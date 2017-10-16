/** A type that can be converted into a request body. */
pub struct SyncBody(Body);

impl SyncBody {
    /** Convert the body into its inner value. */
    pub fn into_inner(self) -> Body {
        self.0
    }
}

impl From<Body> for SyncBody {
    fn from(body: Body) -> SyncBody {
        SyncBody(body)
    }
}

impl From<File> for SyncBody {
    fn from(body: File) -> SyncBody {
        SyncBody(body.into())
    }
}

impl From<Vec<u8>> for SyncBody {
    fn from(body: Vec<u8>) -> SyncBody {
        SyncBody(body.into())
    }
}

impl From<String> for SyncBody {
    fn from(body: String) -> SyncBody {
        SyncBody(body.into())
    }
}

impl From<Value> for SyncBody {
    fn from(body: Value) -> SyncBody {
        SyncBody(body.to_string().into())
    }
}

impl From<&'static [u8]> for SyncBody {
    fn from(body: &'static [u8]) -> SyncBody {
        SyncBody(Body::new(Cursor::new(body)))
    }
}

impl From<&'static str> for SyncBody {
    fn from(body: &'static str) -> SyncBody {
        SyncBody(Body::new(Cursor::new(body)))
    }
}
