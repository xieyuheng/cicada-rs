use std::ops::{
    AddAssign, MulAssign, Neg,
};

use serde::de::{
    self, Deserialize,
    DeserializeSeed,
    EnumAccess, IntoDeserializer,
    MapAccess, SeqAccess, VariantAccess, Visitor,
};

use crate::error::{
    Error, Result,
};

pub struct Deserializer <'de> {
    input: &'de str,
}

impl <'de> Deserializer <'de> {
    pub fn from_str (input: &'de str) -> Self {
        Deserializer { input }
    }
}

pub fn from_str <'a, T> (s: &'a str) -> Result <T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_str (s);
    let t = T::deserialize (&mut deserializer)?;
    if deserializer.input.is_empty () {
        Ok (t)
    } else {
        Err (Error::TrailingCharacters)
    }
}

impl <'de> Deserializer <'de> {
    fn peek_char (&mut self) -> Result <char> {
        self.input
            .chars ()
            .next ()
            .ok_or (Error::Eof)
    }

    fn next_char (&mut self) -> Result <char> {
        let ch = self.peek_char ()?;
        self.input = &self.input [ch.len_utf8 () ..];
        Ok (ch)
    }

    fn parse_bool (&mut self) -> Result <bool> {
        if self.input.starts_with ("true") {
            self.input = &self.input ["true".len () ..];
            Ok (true)
        } else if self.input.starts_with ("false") {
            self.input = &self.input ["false".len () ..];
            Ok (false)
        } else {
            Err (Error::ExpectedBoolean)
        }
    }

    // parse a group of decimal digits
    //   as an unsigned integer of type T
    fn parse_unsigned <T> (&mut self) -> Result <T>
    where
        T: AddAssign <T> + MulAssign <T> + From <u8>
    {
        let mut int = match self.next_char ()? {
            ch @ '0'...'9' => T::from (ch as u8 - b'0'),
            _ => {
                return Err (Error::ExpectedInteger);
            }
        };
        loop {
            match self.input .chars () .next () {
                Some (ch @ '0'...'9') => {
                    self.input = &self.input [1..];
                    int *= T::from (10);
                    int += T::from (ch as u8 - b'0');
                }
                _ => {
                    return Ok (int);
                }
            }
        }
    }

    // parse a possible minus sign
    //   followed by a group of decimal digits
    //   as a signed integer of type T
    fn parse_signed <T> (&mut self) -> Result <T>
    where
        T: Neg <Output = T> + AddAssign <T> + MulAssign <T> + From <i8>
    {
        unimplemented! ()
    }

    // parse a string until the next '"' character
    //   escape sequences are not handled
    fn parse_string (&mut self) -> Result <&'de str> {
        if self.next_char ()? != '"' {
            return Err (Error::ExpectedString);
        }
        match self.input .find ('"') {
            Some (len) => {
                let s = &self.input [..len];
                self.input = &self.input [len + 1..];
                Ok (s)
            }
            None => Err (Error::Eof),
        }
    }
}

impl <'de, 'a> de::Deserializer <'de>
    for &'a mut Deserializer <'de>
{
    type Error = Error;

    fn deserialize_any <V> (self, visitor: V) -> Result <V::Value>
    where
        V: Visitor <'de>
    {
        match self.peek_char ()? {
            'n' => self.deserialize_unit (visitor),
            't' | 'f' => self.deserialize_bool (visitor),
            '"' => self.deserialize_str (visitor),
            '0'...'9' => self.deserialize_u64 (visitor),
            '-' => self.deserialize_i64 (visitor),
            '[' => self.deserialize_seq (visitor),
            '{' => self.deserialize_map (visitor),
            _ => Err (Error::Syntax),
        }
    }

    fn deserialize_bool <V> (self, visitor: V) -> Result <V::Value>
    where
        V: Visitor <'de>
    {
        visitor.visit_bool (self.parse_bool ()?)
    }

    fn deserialize_i8 <V> (self, visitor: V) -> Result <V::Value>
    where
        V: Visitor <'de>
    {
        visitor.visit_i8 (self.parse_signed ()?)
    }

    fn deserialize_i16 <V> (self, visitor: V) -> Result <V::Value>
    where
        V: Visitor <'de>
    {
        visitor.visit_i16 (self.parse_signed ()?)
    }

    fn deserialize_i32 <V> (self, visitor: V) -> Result <V::Value>
    where
        V: Visitor <'de>
    {
        visitor.visit_i32 (self.parse_signed ()?)
    }

    fn deserialize_i64 <V> (self, visitor: V) -> Result <V::Value>
    where
        V: Visitor <'de>
    {
        visitor.visit_i64 (self.parse_signed ()?)
    }

    fn deserialize_u8 <V> (self, visitor: V) -> Result <V::Value>
    where
        V: Visitor <'de>
    {
        visitor.visit_u8 (self.parse_unsigned ()?)
    }

    fn deserialize_u16 <V> (self, visitor: V) -> Result <V::Value>
    where
        V: Visitor <'de>
    {
        visitor.visit_u16 (self.parse_unsigned ()?)
    }

    fn deserialize_u32 <V> (self, visitor: V) -> Result <V::Value>
    where
        V: Visitor <'de>
    {
        visitor.visit_u32 (self.parse_unsigned ()?)
    }

    fn deserialize_u64 <V> (self, visitor: V) -> Result <V::Value>
    where
        V: Visitor <'de>
    {
        visitor.visit_u64 (self.parse_unsigned ()?)
    }

    fn deserialize_f32 <V> (self, _visitor: V) -> Result <V::Value>
    where
        V: Visitor <'de>
    {
        unimplemented! ()
    }

    fn deserialize_f64 <V> (self, _visitor: V) -> Result <V::Value>
    where
        V: Visitor <'de>
    {
        unimplemented! ()
    }

    fn deserialize_char <V> (self, _visitor: V) -> Result <V::Value>
    where
        V: Visitor <'de>
    {
        // parse a string
        //   check that it is one character
        //   call `visit_char`
        unimplemented! ()
    }

    fn deserialize_str <V> (self, visitor: V) -> Result <V::Value>
    where
        V: Visitor <'de>
    {
        visitor.visit_borrowed_str (self.parse_string ()?)
    }

    fn deserialize_string <V> (self, visitor: V) -> Result <V::Value>
    where
        V: Visitor <'de>
    {
        self.deserialize_str (visitor)
    }

    fn deserialize_bytes <V> (self, _visitor: V) -> Result <V::Value>
    where
        V: Visitor <'de>
    {
        unimplemented! ()
    }

    fn deserialize_byte_buf <V> (self, _visitor: V) -> Result <V::Value>
    where
        V: Visitor <'de>
    {
        unimplemented! ()
    }

    fn deserialize_option <V> (
        self,
        visitor: V,
    ) -> Result <V::Value>
    where
        V: Visitor <'de>
    {
        if self.input.starts_with ("None") {
            self.input = &self.input ["None".len () ..];
            visitor.visit_none ()
        } else {
            visitor.visit_some (self)
        }
    }

    fn deserialize_unit <V> (
        self,
        visitor: V,
    ) -> Result <V::Value>
    where
        V: Visitor <'de>
    {
        if self.input.starts_with ("()") {
            self.input = &self.input ["()".len () ..];
            visitor.visit_unit ()
        } else {
            Err (Error::ExpectedNull)
        }
    }

    // Unit struct means a named value containing no data.
    fn deserialize_unit_struct <V> (
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result <V::Value>
    where
        V: Visitor <'de>
    {
        self.deserialize_unit (visitor)
    }

    // As is done here, serializers are encouraged to treat newtype structs as
    // insignificant wrappers around the data they contain. That means not
    // parsing anything other than the contained value.
    fn deserialize_newtype_struct <V> (
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result <V::Value>
    where
        V: Visitor <'de>
    {
        visitor.visit_newtype_struct (self)
    }

    // Deserialization of compound types like sequences and maps happens by
    // passing the visitor an "Access" object that gives it the ability to
    // iterate through the data contained in the sequence.
    fn deserialize_seq <V> (
        mut self,
        visitor: V,
    ) -> Result <V::Value>
    where
        V: Visitor <'de>
    {
        // Parse the opening bracket of the sequence.
        if self.next_char ()? == '[' {
            // Give the visitor access to each element of the sequence.
            let value = visitor.visit_seq(CommaSeparated::new(&mut self))?;
            // Parse the closing bracket of the sequence.
            if self.next_char ()? == ']' {
                Ok (value)
            } else {
                Err (Error::ExpectedArrayEnd)
            }
        } else {
            Err (Error::ExpectedArray)
        }
    }

    // Tuples look just like sequences in JSON. Some formats may be able to
    // represent tuples more efficiently.
    //
    // As indicated by the length parameter, the `Deserialize` implementation
    // for a tuple in the Serde data model is required to know the length of the
    // tuple before even looking at the input data.
    fn deserialize_tuple <V> (self, _len: usize, visitor: V) -> Result <V::Value>
    where
        V: Visitor <'de>
    {
        self.deserialize_seq (visitor)
    }

    // Tuple structs look just like sequences in JSON.
    fn deserialize_tuple_struct <V> (
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result <V::Value>
    where
        V: Visitor <'de>
    {
        self.deserialize_seq (visitor)
    }

    // Much like `deserialize_seq` but calls the visitors `visit_map` method
    // with a `MapAccess` implementation, rather than the visitor's `visit_seq`
    // method with a `SeqAccess` implementation.
    fn deserialize_map <V> (
        mut self,
        visitor: V,
    ) -> Result <V::Value>
    where
        V: Visitor <'de>
    {
        // Parse the opening brace of the map.
        if self.next_char ()? == '{' {
            // Give the visitor access to each entry of the map.
            let value = visitor.visit_map (
                CommaSeparated::new (&mut self))?;
            // Parse the closing brace of the map.
            if self.next_char ()? == '}' {
                Ok (value)
            } else {
                Err (Error::ExpectedMapEnd)
            }
        } else {
            Err (Error::ExpectedMap)
        }
    }

    // Structs look just like maps in JSON.
    //
    // Notice the `fields` parameter - a "struct" in the Serde data model means
    // that the `Deserialize` implementation is required to know what the fields
    // are before even looking at the input data. Any key-value pairing in which
    // the fields cannot be known ahead of time is probably a map.
    fn deserialize_struct <V> (
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result <V::Value>
    where
        V: Visitor <'de>
    {
        println! ("- deserialize_struct");
        println! ("  name : {}", name);
        println! ("  fields : {:?}", fields);
        self.deserialize_map (visitor)
    }

    fn deserialize_enum <V> (
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result <V::Value>
    where
        V: Visitor <'de>
    {
        println! ("- deserialize_enum");
        println! ("  name : {}", name);
        println! ("  variants : {:?}", variants);
        if self.peek_char ()? == '"' {
            // Visit a unit variant.
            visitor.visit_enum(
                self.parse_string ()?
                    .into_deserializer ())
        } else if self.next_char ()? == '{' {
            // Visit a newtype variant, tuple variant, or struct variant.
            let value = visitor.visit_enum(Enum::new(self))?;
            // Parse the matching close brace.
            if self.next_char ()? == '}' {
                Ok (value)
            } else {
                Err (Error::ExpectedMapEnd)
            }
        } else {
            Err (Error::ExpectedEnum)
        }
    }

    // An identifier in Serde is the type that identifies a field of a struct or
    // the variant of an enum. In JSON, struct fields and enum variants are
    // represented as strings. In other formats they may be represented as
    // numeric indices.
    fn deserialize_identifier <V> (
        self,
        visitor: V,
    ) -> Result <V::Value>
    where
        V: Visitor <'de>
    {
        self.deserialize_str (visitor)
    }

    // Like `deserialize_any` but indicates to the `Deserializer` that it makes
    // no difference which `Visitor` method is called because the data is
    // ignored.
    //
    // Some deserializers are able to implement this more efficiently than
    // `deserialize_any`, for example by rapidly skipping over matched
    // delimiters without paying close attention to the data in between.
    //
    // Some formats are not able to implement this at all. Formats that can
    // implement `deserialize_any` and `deserialize_ignored_any` are known as
    // self-describing.
    fn deserialize_ignored_any <V> (
        self,
        visitor: V,
    ) -> Result <V::Value>
    where
        V: Visitor <'de>
    {
        self.deserialize_any (visitor)
    }
}

// In order to handle commas correctly when deserializing a JSON array or map,
// we need to track whether we are on the first element or past the first
// element.
struct CommaSeparated<'a, 'de: 'a> {
    de: &'a mut Deserializer <'de>,
    first: bool,
}

impl <'a, 'de> CommaSeparated<'a, 'de> {
    fn new(de: &'a mut Deserializer <'de>) -> Self {
        CommaSeparated {
            de,
            first: true,
        }
    }
}

// `SeqAccess` is provided to the `Visitor` to give it the ability to iterate
// through elements of the sequence.
impl <'de, 'a> SeqAccess<'de> for CommaSeparated<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T> (&mut self, seed: T) -> Result <Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        // Check if there are no more elements.
        if self.de.peek_char ()? == ']' {
            return Ok (None);
        }
        // Comma is required before every element except the first.
        if !self.first && self.de.next_char ()? != ',' {
            return Err (Error::ExpectedArrayComma);
        }
        self.first = false;
        // Deserialize an array element.
        seed.deserialize(&mut *self.de) .map (Some)
    }
}

// `MapAccess` is provided to the `Visitor` to give it the ability to iterate
// through entries of the map.
impl <'de, 'a> MapAccess<'de> for CommaSeparated<'a, 'de> {
    type Error = Error;

    fn next_key_seed<K> (&mut self, seed: K) -> Result <Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        // Check if there are no more entries.
        if self.de.peek_char ()? == '}' {
            return Ok (None);
        }
        // Comma is required before every entry except the first.
        if !self.first && self.de.next_char ()? != ',' {
            return Err (Error::ExpectedMapComma);
        }
        self.first = false;
        // Deserialize a map key.
        seed.deserialize(&mut *self.de) .map (Some)
    }

    fn next_value_seed<V> (&mut self, seed: V) -> Result <V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        // It doesn't make a difference whether the colon is parsed at the end
        // of `next_key_seed` or at the beginning of `next_value_seed`. In this
        // case the code is a bit simpler having it here.
        if self.de.next_char ()? != ':' {
            return Err (Error::ExpectedMapColon);
        }
        // Deserialize a map value.
        seed.deserialize(&mut *self.de)
    }
}


struct Enum<'a, 'de: 'a> {
    de: &'a mut Deserializer <'de>,
}

impl <'a, 'de> Enum<'a, 'de> {
    fn new(de: &'a mut Deserializer <'de>) -> Self {
        Enum { de }
    }
}

// `EnumAccess` is provided to the `Visitor` to give it the ability to determine
// which variant of the enum is supposed to be deserialized.
//
// Note that all enum deserialization methods in Serde refer exclusively to the
// "externally tagged" enum representation.
impl <'de, 'a> EnumAccess<'de> for Enum<'a, 'de> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V> (self, seed: V) -> Result <(V::Value, Self::Variant)>
    where
        V: DeserializeSeed<'de>,
    {
        // The `deserialize_enum` method parsed a `{` character so we are
        // currently inside of a map. The seed will be deserializing itself from
        // the key of the map.
        let val = seed.deserialize(&mut *self.de)?;
        // Parse the colon separating map key from value.
        if self.de.next_char ()? == ':' {
            Ok ((val, self))
        } else {
            Err (Error::ExpectedMapColon)
        }
    }
}

// `VariantAccess` is provided to the `Visitor` to give it the ability to see
// the content of the single variant that it decided to deserialize.
impl <'de, 'a> VariantAccess<'de> for Enum<'a, 'de> {
    type Error = Error;

    // If the `Visitor` expected this variant to be a unit variant, the input
    // should have been the plain string case handled in `deserialize_enum`.
    fn unit_variant(self) -> Result <()> {
        Err (Error::ExpectedString)
    }

    // Newtype variants are represented in JSON as `{ NAME: VALUE }` so
    // deserialize the value here.
    fn newtype_variant_seed<T> (self, seed: T) -> Result <T::Value>
    where
        T: DeserializeSeed<'de>,
    {
        seed.deserialize(self.de)
    }

    // Tuple variants are represented in JSON as `{ NAME: [DATA...] }` so
    // deserialize the sequence of data here.
    fn tuple_variant <V> (self, _len: usize, visitor: V) -> Result <V::Value>
    where
        V: Visitor <'de>,
    {
        de::Deserializer::deserialize_seq(self.de, visitor)
    }

    // Struct variants are represented in JSON as `{ NAME: { K: V, ... } }` so
    // deserialize the inner map here.
    fn struct_variant <V> (
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result <V::Value>
    where
        V: Visitor <'de>,
    {
        de::Deserializer::deserialize_map (self.de, visitor)
    }
}

#[cfg(test)]
use serde_derive::{
    Deserialize,
};

#[test]
fn test_primitive () {
    println! ("- de : {:?}", from_str::<bool> ("true") .unwrap ());
    println! ("- de : {:?}", from_str::<bool> ("false") .unwrap ());
    println! ("- de : {:?}", from_str::<()> ("()") .unwrap ());
}

#[test]
fn test_struct () {
    #[derive (Deserialize, PartialEq, Debug)]
    struct Test {
        int: u32,
        seq: Vec <String>,
    }
    let j = r#"{"int":1,"seq":["a","b"]}"#;
    let expected = Test {
        int: 1,
        seq: vec![
            "a".to_string (),
            "b".to_string (),
        ],
    };
    assert_eq! (expected, from_str (j) .unwrap ());
    println! ("- de : {:?}", from_str::<Test> (j) .unwrap ());
}

#[test]
fn test_enum () {
    #[derive (Deserialize, PartialEq, Debug)]
    enum E {
        Unit,
        Newtype (u32),
        Tuple (u32, u32),
        Struct { a: u32 },
    }

    let j = r#""Unit""#;
    let expected = E::Unit;
    println! ("- de : {:?}", from_str::<E> (j) .unwrap ()) ;
    assert_eq! (expected, from_str (j) .unwrap ());

    let j = r#"{"Newtype":1}"#;
    let expected = E::Newtype (1);
    assert_eq! (expected, from_str (j) .unwrap ());

    let j = r#"{"Tuple":[1,2]}"#;
    let expected = E::Tuple (1, 2);
    assert_eq! (expected, from_str (j) .unwrap ());

    let j = r#"{"Struct":{"a":1}}"#;
    let expected = E::Struct { a: 1 };
    assert_eq! (expected, from_str (j) .unwrap ());
}
