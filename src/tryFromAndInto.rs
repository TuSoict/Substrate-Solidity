use std::convert::TryFrom;

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}



pub fn example() {

    // TryFrom
    println!("{:?}", EvenNumber::try_from(10));
    assert_eq!(EvenNumber::try_from(10), Ok(EvenNumber(10)));  // if error => panic 


    // // TryInto
    let result: Result<EvenNumber, ()> = 9i32.try_into();   //
    assert_eq!(result, Err(()));
    assert_eq!(result, Ok(EvenNumber(8)));

    
}


