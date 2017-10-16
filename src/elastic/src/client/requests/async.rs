/** A type that can be converted into a request body. */
pub struct AsyncBody(Body);

impl AsyncBody {
    /** Convert the body into its inner value. */
    pub fn into_inner(self) -> Body {
        self.0
    }
}

impl From<Body> for AsyncBody {
    fn from(body: Body) -> AsyncBody {
        AsyncBody(body)
    }
}

impl From<Vec<u8>> for AsyncBody {
    fn from(body: Vec<u8>) -> AsyncBody {
        AsyncBody(body.into())
    }
}

impl From<String> for AsyncBody {
    fn from(body: String) -> AsyncBody {
        AsyncBody(body.into())
    }
}

impl From<Value> for AsyncBody {
    fn from(body: Value) -> AsyncBody {
        AsyncBody(body.to_string().into())
    }
}

impl From<&'static [u8]> for AsyncBody {
    fn from(body: &'static [u8]) -> AsyncBody {
        AsyncBody(Bytes::from(body).into())
    }
}

impl From<&'static str> for AsyncBody {
    fn from(body: &'static str) -> AsyncBody {
        AsyncBody(Bytes::from(body).into())
    }
}
