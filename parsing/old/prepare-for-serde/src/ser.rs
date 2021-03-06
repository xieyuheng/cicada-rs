use serde::ser::{
    self, Serialize,
};

use crate::error::{
    Error, Result,
};

pub struct Serializer {
    output: String,
}

pub fn to_string <T> (value: &T) -> Result <String>
where
    T: Serialize
{
    let mut serializer = Serializer {
        output: String::new (),
    };
    value.serialize (&mut serializer)?;
    Ok (serializer.output)
}

impl <'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool (self, v: bool) -> Result <()> {
        self.output += if v { "true" } else { "false" };
        Ok (())
    }

    // all int as i64

    fn serialize_i8 (self, v: i8) -> Result <()> {
        self.serialize_i64 (i64::from (v))
    }

    fn serialize_i16 (self, v: i16) -> Result <()> {
        self.serialize_i64 (i64::from (v))
    }

    fn serialize_i32 (self, v: i32) -> Result <()> {
        self.serialize_i64 (i64::from (v))
    }

    fn serialize_i64 (self, v: i64) -> Result <()> {
        self.output += &v.to_string ();
        Ok (())
    }

    // all unsigned int as u64

    fn serialize_u8 (self, v: u8) -> Result <()> {
        self.serialize_u64 (u64::from (v))
    }

    fn serialize_u16 (self, v: u16) -> Result <()> {
        self.serialize_u64 (u64::from (v))
    }

    fn serialize_u32 (self, v: u32) -> Result <()> {
        self.serialize_u64 (u64::from (v))
    }

    fn serialize_u64 (self, v: u64) -> Result <()> {
        self.output += &v.to_string ();
        Ok (())
    }

    fn serialize_f32 (self, v: f32) -> Result <()> {
        self.serialize_f64 (f64::from (v))
    }

    fn serialize_f64 (self, v: f64) -> Result <()> {
        self.output += &v.to_string ();
        Ok (())
    }

    // char as a single-character string
    fn serialize_char (self, v: char) -> Result <()> {
        self.serialize_str (&v.to_string ())
    }

    // escape sequences are not handled
    fn serialize_str (self, v: &str) -> Result <()> {
        self.output += "\"";
        self.output += v;
        self.output += "\"";
        Ok (())
    }

    // serialize a byte array as an array of bytes
    //   could also use a base64 string here
    fn serialize_bytes (self, v: &[u8]) -> Result <()> {
        use serde::ser::SerializeSeq;
        let mut seq = self.serialize_seq (Some (v.len ()))?;
        for byte in v {
            seq.serialize_element (byte)?;
        }
        seq.end ()
    }

    fn serialize_none (self) -> Result <()> {
        self.output += "None";
        Ok (())
    }

    fn serialize_some <T> (self, value: &T) -> Result <()>
    where
        T: ?Sized + Serialize
    {
        self.output += "Some (";
        value.serialize (&mut *self)?;
        self.output += ")";
        Ok (())
    }

    fn serialize_unit (self) -> Result <()> {
        self.output += "()";
        Ok (())
    }

    fn serialize_unit_struct (
        self,
        name: &'static str,
    ) -> Result <()> {
        self.output += name;
        // self.output += " ()";
        Ok (())
    }

    fn serialize_unit_variant (
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result <()> {
        self.output += name;
        self.output += "::";
        self.output += variant;
        // self.output += " ()";
        Ok (())
    }

    fn serialize_newtype_struct <T> (
        self,
        name: &'static str,
        value: &T,
    ) -> Result <()>
    where
        T: ?Sized + Serialize,
    {
        self.output += name;
        self.output += " (";
        value.serialize (&mut *self)?;
        self.output += ")";
        Ok (())
    }

    fn serialize_newtype_variant <T> (
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result <()>
    where
        T: ?Sized + Serialize,
    {
        self.output += name;
        self.output += "::";
        self.output += variant;
        self.output += " (";
        value.serialize (&mut *self)?;
        self.output += ")";
        Ok (())
    }

    fn serialize_seq (
        self,
        _len: Option <usize>,
    ) -> Result <Self::SerializeSeq> {
        self.output += "[";
        Ok (self)
    }

    fn serialize_tuple (
        self,
        _len: usize,
    ) -> Result <Self::SerializeTuple> {
        Ok (self)
    }

    fn serialize_tuple_struct (
        self,
        name: &'static str,
        len: usize,
    ) -> Result <Self::SerializeTupleStruct> {
        self.output += name;
        self.output += " (";
        self.serialize_tuple (len)
    }

    fn serialize_tuple_variant (
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result <Self::SerializeTupleVariant> {
        self.output += name;
        self.output += "::";
        self.output += variant;
        self.output += " (";
        self.serialize_tuple (len)
    }

    // `{ K: V, K: V, ... }`
    fn serialize_map (
        self,
        _len: Option <usize>,
    ) -> Result <Self::SerializeMap> {
        self.output += "{ ";
        Ok (self)
    }

    fn serialize_struct (
        self,
        name: &'static str,
        len: usize,
    ) -> Result <Self::SerializeStruct> {
        self.output += name;
        self.output += " ";
        self.serialize_map (Some (len))
    }

    fn serialize_struct_variant (
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result <Self::SerializeStructVariant> {
        self.output += name;
        self.output += "::";
        self.output += variant;
        self.output += " ";
        self.serialize_map (Some (len))
    }
}

impl <'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element <T> (&mut self, value: &T) -> Result <()>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with ("[") {
            self.output += ", ";
        }
        value.serialize (&mut **self)
    }

    fn end (self) -> Result <()> {
        self.output += "]";
        Ok (())
    }
}

impl <'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element <T> (&mut self, value: &T) -> Result <()>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with ("(") {
            self.output += ", ";
        }
        value.serialize (&mut **self)
    }

    fn end(self) -> Result <()> {
        self.output += ")";
        Ok (())
    }
}

impl <'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T> (&mut self, value: &T) -> Result <()>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with ("(") {
            self.output += ", ";
        }
        value.serialize (&mut **self)
    }

    fn end(self) -> Result <()> {
        self.output += ")";
        Ok (())
    }
}

impl <'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field <T> (&mut self, value: &T) -> Result <()>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with ("(") {
            self.output += ", ";
        }
        value.serialize (&mut **self)
    }

    fn end(self) -> Result <()> {
        self.output += ")";
        Ok (())
    }
}

impl <'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key <T> (&mut self, key: &T) -> Result <()>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with ("{ ") {
            self.output += ", ";
        }
        key.serialize (&mut **self)
    }

    fn serialize_value <T> (&mut self, value: &T) -> Result <()>
    where
        T: ?Sized + Serialize,
    {
        self.output += ": ";
        value.serialize (&mut **self)
    }

    fn end (self) -> Result <()> {
        self.output += " }";
        Ok (())
    }
}

// structs are like maps in which the keys are str
impl <'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field <T> (
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result <()>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with ("{ ") {
            self.output += ", ";
        }
        self.output += key;
        self.output += ": ";
        value.serialize (&mut **self)
    }

    fn end (self) -> Result <()> {
        self.output += " }";
        Ok (())
    }
}

impl <'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field <T> (
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result <()>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with ("{ ") {
            self.output += ", ";
        }
        self.output += key;
        self.output += ": ";
        value.serialize (&mut **self)
    }

    fn end (self) -> Result <()> {
        self.output += " }";
        Ok (())
    }
}

#[cfg(test)]
use serde_derive::{
    Serialize,
};

#[test]
fn test_struct () {
    #[derive (Serialize)]
    struct UnitStruct;
    println! ("- ser : {}", to_string (&UnitStruct) .unwrap ());

    #[derive (Serialize)]
    struct NewStruct (UnitStruct);
    println! ("- ser : {}", to_string (&NewStruct (UnitStruct)) .unwrap ());

    #[derive (Serialize)]
    struct Struct1 {
        int: u32,
        seq: Vec <&'static str>,
    }
    let struct1 = Struct1 {
        int: 1,
        seq: vec! ["a", "b"],
    };
    println! ("- ser : {}", to_string (&struct1) .unwrap ());
}

#[test]
fn test_primitive () {
    println! ("- ser : {}", to_string (&()) .unwrap ());
    println! ("- ser : {}", to_string (&1234) .unwrap ());
    println! ("- ser : {}", to_string (&3.14) .unwrap ());
    println! ("- ser : {}", to_string (&"1234") .unwrap ());
    println! ("- ser : {}", to_string (&String::from ("1234")) .unwrap ());
}

#[test]
fn test_option () {
    println! ("- ser : {}", to_string (&None::<String>) .unwrap ());
    println! ("- ser : {}", to_string (&Some (1234)) .unwrap ());
    println! ("- ser : {}", to_string (&Some ("1234")) .unwrap ());
    println! ("- ser : {}", to_string (&Some (String::from ("1234"))) .unwrap ());
}

#[test]
fn test_enum () {
    #[derive (Serialize)]
    enum E {
        Unit,
        Newtype (u32),
        Tuple (u32, u32),
        Struct { a: u32 },
    }
    let u = E::Unit;
    println! ("- ser : {}", to_string (&u) .unwrap ());
    let n = E::Newtype (1);
    println! ("- ser : {}", to_string (&n) .unwrap ());
    let t = E::Tuple (1, 2);
    println! ("- ser : {}", to_string (&t) .unwrap ());
    let s = E::Struct { a: 1 };
    println! ("- ser : {}", to_string (&s) .unwrap ());
}
