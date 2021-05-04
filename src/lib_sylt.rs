use crate::*;
use crate as sylt;

#[sylt_macro::sylt_link(push, "sylt::lib_sylt")]
pub fn dbg(values: &[Value], _typecheck: bool) -> Result<Value, RuntimeError> {
    println!(
        "{}: {:?}, {:?}",
        "DBG".purple(),
        values.iter().map(Type::from).collect::<Vec<_>>(),
        values
    );
    Ok(Value::Nil)
}

#[sylt_macro::sylt_link(push, "sylt::lib_sylt")]
pub fn push(values: &[Value], typecheck: bool) -> Result<Value, RuntimeError> {
    match (values, typecheck) {
        ([Value::List(ls), v], true) => {
            let ls: &RefCell<_> = ls.borrow();
            let ls = &ls.borrow();
            assert!(ls.len() == 1);
            let ls = Type::from(&ls[0]);
            let v: Type = Type::from(&*v);
            if ls == v {
                Ok(Value::Nil)
            } else {
                Err(RuntimeError::TypeMismatch(ls, v))
            }
        }
        ([Value::List(ls), v], false) => {
            // NOTE(ed): Deliberately no type checking.
            let ls: &RefCell<_> = ls.borrow();
            ls.borrow_mut().push(v.clone());
            Ok(Value::Nil)
        }
        (values, _) => Err(RuntimeError::ExternTypeMismatch(
            "push".to_string(),
            values.iter().map(Type::from).collect(),
        )),
    }
}

#[sylt_macro::sylt_link(prepend, "sylt::lib_sylt")]
pub fn prepend(values: &[Value], typecheck: bool) -> Result<Value, RuntimeError> {
    match (values, typecheck) {
        ([Value::List(ls), v], true) => {
            let ls: &RefCell<_> = ls.borrow();
            let ls = &ls.borrow();
            assert!(ls.len() == 1);
            let ls = Type::from(&ls[0]);
            let v: Type = Type::from(&*v);
            if ls == v {
                Ok(Value::Nil)
            } else {
                Err(RuntimeError::TypeMismatch(ls, v))
            }
        }
        ([Value::List(ls), v], false) => {
            // NOTE(ed): Deliberately no type checking.
            let ls: &RefCell<_> = ls.borrow();
            ls.borrow_mut().insert(0, v.clone());
            Ok(Value::Nil)
        }
        (values, _) => Err(RuntimeError::ExternTypeMismatch(
            "prepend".to_string(),
            values.iter().map(Type::from).collect(),
        )),
    }
}

#[sylt_macro::sylt_link(len, "sylt::lib_sylt")]
pub fn len(values: &[Value], _: bool) -> Result<Value, RuntimeError> {
    match values {
        [Value::Tuple(ls)] => {
            Ok(Value::Int(ls.len() as i64))
        }
        [Value::List(ls)] => {
            Ok(Value::Int(RefCell::borrow(ls).len() as i64))
        }
        values => Err(RuntimeError::ExternTypeMismatch(
            "len".to_string(),
            values.iter().map(Type::from).collect(),
        )),
    }
}

sylt_macro::extern_function!(
    "sylt::lib_sylt"
    sin
    [One(Float(t))] -> Type::Float => {
        Ok(Float(t.sin()))
    },
);

sylt_macro::extern_function!(
    "sylt::lib_sylt"
    cos
    [One(Float(t))] -> Type::Float => {
        Ok(Float(t.cos()))
    },
);

sylt_macro::extern_function!(
    "sylt::lib_sylt"
    as_float
    [One(Int(t))] -> Type::Float => {
        Ok(Float(*t as f64))
    },
);

pub fn union_type(a: Type, b: Type) -> Type{
    if a.fits(&b) {
        a
    } else if b.fits(&a) {
        b
    } else {
        match (a, b) {
            (Type::Union(a), Type::Union(b)) => {
                Type::Union(a.union(&b).cloned().collect())
            }
            (b, Type::Union(a)) | (Type::Union(a), b) => {
                let mut a = a.clone();
                a.insert(b.clone());
                Type::Union(a)
            }
            (a, b) => {
                Type::Union([a, b].iter().cloned().collect())
            }
        }
    }
}

#[sylt_macro::sylt_link(pop, "sylt::lib_sylt")]
pub fn pop(values: &[Value], typecheck: bool) -> Result<Value, RuntimeError> {
    match (values, typecheck) {
        ([Value::List(ls)], true) => {
            let ls: &RefCell<_> = ls.borrow();
            let ls = &ls.borrow();
            // TODO(ed): Write correct typing
            let ls = Type::from(&ls[0]);
            let ret = union_type(ls, Type::Void);
            Ok(Value::from(ret))
        }
        ([Value::List(ls)], false) => {
            // NOTE(ed): Deliberately no type checking.
            let ls: &RefCell<_> = ls.borrow();
            let last = ls.borrow_mut().pop().unwrap_or(Value::Nil);
            Ok(last)
        }
        (values, _) => Err(RuntimeError::ExternTypeMismatch(
            "pop".to_string(),
            values.iter().map(Type::from).collect(),
        )),
    }
}


#[sylt_macro::sylt_link(inf, "sylt::lib_sylt")]
pub fn inf(values: &[Value], _typecheck: bool) -> Result<Value, RuntimeError> {
    match values {
        [x] => {
            let t: Type = Type::from(&*x);
            let x = x.clone();
            Ok(Value::Iter(t, Rc::new(RefCell::new(move || Some(x.clone())))))
        }
        values => Err(RuntimeError::ExternTypeMismatch(
            "inf".to_string(),
            values.iter().map(Type::from).collect(),
        )),
    }
}


sylt_macro::sylt_link_gen!("sylt::lib_sylt");
