

error_chain! {
//        foreign_links {
//            AddrParseError(::std::net::AddrParseError);
//            ParseIntError(::std::num::ParseIntError);
//        }

        errors {
            UnknownFilter(v: String) {
                description("unknown filter type"), // note the ,
                display("unknown filter type: '{}'", v), // trailing comma is allowed
            }

            UnknownOperator(v: String) {
                description("unknown operator"), // note the ,
                display("unknown operator: '{}'", v), // trailing comma is allowed
            }
        }
    }

