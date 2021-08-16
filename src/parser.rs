use std::rc::Rc;

pub trait Parser {
    type Out;

    fn call<'a>(&self, bytes: &'a[u8]) -> Option<(Self::Out, &'a[u8])>;


    fn map<B, F>(self, func: F) -> Map<Self, F> 
    where
        Self: Sized,
        F: Fn(<Self as Parser>::Out) -> B
    {
        Map::new(self, func)
    }


    fn flat_map<P, F>(self, func: F) -> FlatMap<Self, F> 
    where
        Self: Sized,
        P: Parser,
        F: Fn(<Self as Parser>::Out) -> P 
    {
        FlatMap::new(self, func)
    }
}


pub struct Map<P, F> {
    parser: P,
    func: F 
}


impl<P, F, A> Map<P, F>
where
    P: Parser,
    F: Fn(<P as Parser>::Out) -> A 
{
    pub fn new(parser: P, func: F) -> Self {
        Self {
            parser,
            func
        }
    }
}


impl<P, F, A> Parser for Map<P, F>  
where
    P: Parser,
    F: Fn(<P as Parser>::Out) -> A
{
    type Out = A;

    fn call<'a>(&self, input: &'a [u8]) -> Option<(Self::Out, &'a [u8])> {
        self.parser.call(input).map(move |(a, b)| {
            ((self.func)(a), b)
        })
    }
}


pub struct FlatMap<P, F> {
    parser: P,
    func: F
}


impl<P, F, Q> FlatMap<P, F> 
where
    P: Parser,
    Q: Parser,
    F: Fn(<P as Parser>::Out) -> Q,
{
    pub fn new(parser: P, func: F) -> Self {
        Self {
            parser,
            func
        }
    }
}


impl<P, F, Q> Parser for FlatMap<P, F> 
where
    P: Parser,
    Q: Parser,
    F: Fn(<P as Parser>::Out) -> Q 
{
    type Out = <Q as Parser>::Out;

    fn call<'a>(&self, input: &'a [u8]) -> Option<(Self::Out, &'a [u8])> {
        self.parser.call(input).and_then(|(a, b)| {
            (self.func)(a).call(b)
        })
    }
}

pub struct Success<A> {
    data: Rc<A>
}


impl<A> Success<A> {
    pub fn new(data: A) -> Self {
        Self { data: Rc::new(data) }
    }
}


impl<A> Parser for Success<A> {
    type Out = Rc<A>;

    fn call<'b>(&self, input: &'b [u8]) -> Option<(Self::Out, &'b [u8])> {
        Some((self.data.clone(), input))
    }
}

pub struct Item {}

impl Item {
    pub fn new() -> Self {
        Self {}
    }
}

impl Parser for Item {
    type Out = u8;

    fn call<'a>(&self, input: &'a [u8]) -> Option<(Self::Out, &'a [u8])> {
        input.get(0).map(|b| (*b, &input[1..]))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_test() {
        let stuff = "hello world".as_bytes();

        let (result, _) = 
            Item::new()
                .map(|x| format!("hi, {}", x))
                .call(&stuff)
                .unwrap();

        assert_eq!("hi, 104", result);
    }

    #[test]
    fn bind_test() {
        let stuff = "hello world".as_bytes();

        let (result, _) = 
            Item::new()
                .flat_map(|x| Success::new(x))
                .call(&stuff)
                .unwrap();

        assert_eq!(104, *result);
    }

    // #[test]
    // fn take_test() {
    //     let stuff = "hello world";

    //     let (result, rest) = 
    //         Take::new(3, Item::new())
    //             .call(&stuff)
    //             .unwrap();

    //     assert_eq!(vec!['h', 'e', 'l'], result);
    //     assert_eq!("lo world", rest);
    // }


    // #[test]
    // fn probably_not_needed_take_test() {
    //     let stuff = "hello world";

    //     let (result, rest) = 
    //         Take::new(3, Item::new())
    //             .map(|it| it.len()) 
    //             .call(&stuff)
    //             .unwrap();

    //     assert_eq!(3, result);
    //     assert_eq!("lo world", rest);
    // }

    // #[test]
    // fn another_probably_not_needed_take_test() {
    //     let stuff = "hello world";

    //     let (result, rest) = 
    //         Take::new(3, Item::new())
    //             .map(|it| it.iter().map(|c| format!("{}!", c)).collect::<Vec<String>>()) 
    //             .call(&stuff)
    //             .unwrap();

    //     assert_eq!(vec!["h!".to_string(), "e!".to_string(), "l!".to_string()], result);
    //     assert_eq!("lo world", rest);
    // }

    // #[test]
    // fn another_another_probably_not_needed_take_test() {
    //     let stuff = "hello world";

    //     let (result, rest) = 
    //         Item::new()
    //             .take(3)
    //             .map(|it| it.iter().map(|c| format!("{}!", c)).collect::<Vec<String>>()) 
    //             .call(&stuff)
    //             .unwrap();

    //     assert_eq!(vec!["h!".to_string(), "e!".to_string(), "l!".to_string()], result);
    //     assert_eq!("lo world", rest);
    // }
}