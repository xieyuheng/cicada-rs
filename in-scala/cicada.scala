trait obj_t {

}

class Id

class var_t (
  id: Id,
  name: String,
) extends obj_t {

}

class typed_var_t (
    id: Id,
    name: String,
    ty: obj_t,
) extends obj_t {

}

class disj_t (
    name: String,
    sub_names: List [String],
    body: Map [String, obj_t],
) extends obj_t

class conj_t (
    name: String,
    body: Map [String, obj_t],
) extends obj_t

class data_t (
    name: String,
    body: Map [String, obj_t],
) extends obj_t

class type_of_type_t (
) extends obj_t

class module_t {
  def main (args: Array [String]): Unit =
    println ("cicada")
}
