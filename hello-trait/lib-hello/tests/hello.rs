
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let mine = MyType { data: 0 };
        assert_eq!(mine, mine.foo());
    }
}
//let mine = Box::new(MyType { data: 0 });
//println!("{}", mine.foo());
