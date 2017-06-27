fn testcase1 (){
    //INFIX: 3 * 4 + 6 + 4 * 2
    //POSTFIX: 3 4 * 6 + 4 2 * +
        let x = &[InfixToken::Operand(3),
                  InfixToken::Operator(Operator::Mul),
                  InfixToken::Operand(4),
                  InfixToken::Operator(Operator::Add),
                  InfixToken::Operand(6),
                  InfixToken::Operator(Operator::Add),
                  InfixToken::Operand(4),
                  InfixToken::Operator(Operator::Mul),
                  InfixToken::Operand(2),
              ];
        let y = Some(vec![
                 PostfixToken::Operand(3),
                 PostfixToken::Operand(4),
                 PostfixToken::Operator(Operator::Mul),
                 PostfixToken::Operand(6),
                 PostfixToken::Operator(Operator::Add),
                 PostfixToken::Operand(4),
                 PostfixToken::Operand(2),
                 PostfixToken::Operator(Operator::Mul),
                 PostfixToken::Operator(Operator::Add)]);
        assert_eq!(y, infix_to_postfix(x));
}

fn testcase2(){
    /* INFIX: 3 - 4 + 6 - 4 + 6 * 10
       POSTFIX: 3 4 - 6 + 4 - 6 10 * +
    */
let x = &[InfixToken::Operand(3),
          InfixToken::Operator(Operator::Sub),
          InfixToken::Operand(4),
          InfixToken::Operator(Operator::Add),
          InfixToken::Operand(6),
          InfixToken::Operator(Operator::Sub),
          InfixToken::Operand(4),
          InfixToken::Operator(Operator::Add),
          InfixToken::Operand(6),
          InfixToken::Operator(Operator::Mul),
          InfixToken::Operand(10),
      ];
      let y = Some(vec![
         PostfixToken::Operand(3),
         PostfixToken::Operand(4),
         PostfixToken::Operator(Operator::Sub),
         PostfixToken::Operand(6),
         PostfixToken::Operator(Operator::Add),
         PostfixToken::Operand(4),
         PostfixToken::Operator(Operator::Sub),
         PostfixToken::Operand(6),
         PostfixToken::Operand(10),
         PostfixToken::Operator(Operator::Mul),
         PostfixToken::Operator(Operator::Add)]);
      assert_eq!(y, infix_to_postfix(x));
}
fn testcase3(){
    // INFIX: 6 + 4 + 6 / 10 * 5 * 4
    // POSTFIX: 6 4 + 6 10 / 5 * 4 * +
          let x = &[
              InfixToken::Operand(6),
              InfixToken::Operator(Operator::Add),
              InfixToken::Operand(4),
              InfixToken::Operator(Operator::Add),
              InfixToken::Operand(6),
              InfixToken::Operator(Operator::Div),
              InfixToken::Operand(10),
              InfixToken::Operator(Operator::Mul),
              InfixToken::Operand(5),
              InfixToken::Operator(Operator::Mul),
              InfixToken::Operand(4),
          ];
          let y = Some(vec![
             PostfixToken::Operand(6),
             PostfixToken::Operand(4),
             PostfixToken::Operator(Operator::Add),
             PostfixToken::Operand(6),
             PostfixToken::Operand(10),
             PostfixToken::Operator(Operator::Div),
             PostfixToken::Operand(5),
             PostfixToken::Operator(Operator::Mul),
             PostfixToken::Operand(4),
             PostfixToken::Operator(Operator::Mul),
             PostfixToken::Operator(Operator::Add)]);
          assert_eq!(y, infix_to_postfix(x));


}
fn testcase4 (){
    // INFIX: 3 * (4 + 5)
    // POSTFIX: 3 4 5 + *
let x = &[InfixToken::Operand(3),
          InfixToken::Operator(Operator::Mul),
          InfixToken::LeftParen,
          InfixToken::Operand(4),
          InfixToken::Operator(Operator::Add),
          InfixToken::Operand(5),
          InfixToken::RightParen];
      let y = Some(vec![
         PostfixToken::Operand(3),
         PostfixToken::Operand(4),
         PostfixToken::Operand(5),
         PostfixToken::Operand(10),
         PostfixToken::Operator(Operator::Add),
         PostfixToken::Operator(Operator::Mul)]);
      assert_eq!(y, infix_to_postfix(x));
}
fn testcase5(){
    //INFIX: 3 + (4) + (4)
    //POSTFIX: 3 4 + 4 +
     let x = &[InfixToken::Operand(3),
          InfixToken::Operator(Operator::Add),
          InfixToken::LeftParen,
          InfixToken::Operand(4),
          InfixToken::RightParen,
          InfixToken::Operator(Operator::Add),
          InfixToken::LeftParen,
          InfixToken::Operand(4),
          InfixToken::RightParen,
      ];
      let y = Some(vec![
         PostfixToken::Operand(3),
         PostfixToken::Operand(4),
         PostfixToken::Operator(Operator::Add),
         PostfixToken::Operand(4),
         PostfixToken::Operator(Operator::Add)]);
      assert_eq!(y, infix_to_postfix(x));
}
fn testcase6 (){
    //INFIX: (5+3)-6
    //POSTFIX: 5 3 + 6 -
          let x = &[
              InfixToken::LeftParen,
              InfixToken::Operand(5),
              InfixToken::Operator(Operator::Add),
              InfixToken::Operand(3),
              InfixToken::RightParen,
              InfixToken::Operator(Operator::Sub),
              InfixToken::Operand(6),
          ];
          let y = Some(vec![
             PostfixToken::Operand(5),
             PostfixToken::Operand(3),
             PostfixToken::Operator(Operator::Add),
             PostfixToken::Operand(6),
             PostfixToken::Operator(Operator::Sub)]);
          assert_eq!(y, infix_to_postfix(x));
}
fn testcase7 (){
    //INFIX: 5(5+3)-6
    //POSTFIX: ERROR
    let x = &[
              InfixToken::Operand(5),
              InfixToken::LeftParen,
              InfixToken::Operand(5),
              InfixToken::Operator(Operator::Add),
              InfixToken::Operand(3),
              InfixToken::RightParen,
              InfixToken::Operator(Operator::Sub),
              InfixToken::Operand(6),
          ];
    let y = None;
    assert_eq!(y, infix_to_postfix(x));
}
fn testcase8 (){
    //INFIX: 3 5 6
    //POSTFIX: ERROR
    let x = &[
          InfixToken::Operand(3),
          InfixToken::Operand(5),
          InfixToken::Operand(6),
    ];
    let y = None;
    assert_eq!(y,infix_to_postfix(x));
}
fn testcase9 (){
    //INFX: (3 + 5))
    //POSTFIX: ERROR
    let x= &[
        InfixToken::LeftParen,
        InfixToken::Operand(3),
        InfixToken::Operator(Operator::Add),
        InfixToken::Operand(5),
        InfixToken::RightParen,
        InfixToken::RightParen,
    ];
    let y = None;
    assert_eq!(y,infix_to_postfix(x));
}
fn testcase10 () {
    //INFIX: ((3+5))
    //POSTFIX: 3 5 +
    let x = &[
    InfixToken::LeftParen,
    InfixToken::LeftParen,
    InfixToken::Operand(3),
    InfixToken::Operator(Operator::Add),
    InfixToken::Operand(5),
    InfixToken::RightParen,
    InfixToken::RightParen,
    ];
    let y = Some(vec![
   PostfixToken::Operand(3),
   PostfixToken::Operand(5),
   PostfixToken::Operator(Operator::Add)]);
   assert_eq!(y, infix_to_postfix(x));
}
fn testcase11 (){
    //INFIX: 3 + +
    //POSTFIX: ERROR
    let x = &[
    InfixToken::Operand(3),
    InfixToken::Operator(Operator::Add),
    InfixToken::Operator(Operator::Add)];

    let y = None;
    assert_eq!(y, infix_to_postfix(x));
}
fn testcase12 (){
    //INFX: ((3 + 5)
    //POSTFIX: ERROR
    let x= &[
        InfixToken::LeftParen,
        InfixToken::LeftParen,
        InfixToken::Operand(3),
        InfixToken::Operator(Operator::Add),
        InfixToken::Operand(5),
        InfixToken::RightParen,
    ];
    let y = None;
    assert_eq(y,infix_to_postfix(x));
}
fn testcase13 (){
    //INFIX: (3+5)(3+5)
    //POSTFIX: ERROR
    let x= &[
        InfixToken::LeftParen,
        InfixToken::Operand(3),
        InfixToken::Operator(Operator::Add),
        InfixToken::Operand(5),
        InfixToken::RightParen,
        InfixToken::LeftParen,
        InfixToken::Operand(3),
        InfixToken::Operator(Operator::Add),
        InfixToken::Operand(5),
        InfixToken::RightParen,
    ];
    let y = None;
    assert_eq!(y, infix_to_postfix(x));
}
fn testcase14 (){
    //INFIX: (3+5)5
    //POSTFIX: ERROR
    let x= &[
        InfixToken::LeftParen,
        InfixToken::Operand(3),
        InfixToken::Operator(Operator::Add),
        InfixToken::Operand(5),
        InfixToken::RightParen,
        InfixToken::Operand(5),
    ];
    let y = None;
    assert_eq!(y, infix_to_postfix(x));
}
fn testcase15 (){
    //INFIX: ) 3 + 5 (
    //POSTFIX: Error
    let x= &[
        InfixToken::RightParen,
        InfixToken::Operand(3),
        InfixToken::Operator(Operator::Add),
        InfixToken::Operand(5),
        InfixToken::LeftParen,
    ];
    let y = None;
    assert_eq!(y, infix_to_postfix(x));
}
fn testcase16 (){
    //INFIX: 3 4 +
    //POSTFIX: ERROR
    let x = &[
    InfixToken::Operand(3),
    InfixToken::Operand(4),
    InfixToken::Operator(Operator::Add)];

    let y = None;
    assert_eq!(y, infix_to_postfix(x));
}
fn testcase17 (){
    let x = &[];
    let y = None;
    assert_eq!(y, infix_to_postfix(x));
}
fn testcase18 (){
    //INFIX: +3
    //POSTFIX: ERROR
    let x = &[
    InfixToken::Operator(Operator::Add),
    InfixToken::Operand(3)];

    let y = None;
    assert_eq!(y, infix_to_postfix(x));
}
fn testcase19 (){
    //INFIX: 2+*3
    //POSTFIX: ERROR
    let x = &[
    InfixToken::Operand(3),
    InfixToken::Operator(Operator::Add),
    InfixToken::Operator(Operator::Mul),
    InfixToken::Operand(3)];

    let y = None;
    assert_eq!(y, infix_to_postfix(x));
}
fn testcase20 (){
    //INFIX: +(3+3)
    //POSTFIX: ERROR
    let x = &[
    InfixToken::Operator(Operator::Add),
    InfixToken::LeftParen,
    InfixToken::Operand(3),
    InfixToken::Operator(Operator::Add),
    InfixToken::Operand(3),
    InfixToken::RightParen];

    let y = None;
    assert_eq!(y, infix_to_postfix(x));
}
fn testcase21 (){
    //INFIX: (3+3+)
    //POSTFIX: ERROR
    let x = &[
    InfixToken::LeftParen,
    InfixToken::Operand(3),
    InfixToken::Operator(Operator::Add),
    InfixToken::Operand(3),
    InfixToken::Operator(Operator::Add),
    InfixToken::RightParen];

    let y = None;
    assert_eq!(y, infix_to_postfix(x));

}
fn testcase22 (){
    //INFIX: (3+3+)+3
    //POSTFIX: ERROR
    let x = &[
    InfixToken::LeftParen,
    InfixToken::Operand(3),
    InfixToken::Operator(Operator::Add),
    InfixToken::Operand(3),
    InfixToken::Operator(Operator::Add),
    InfixToken::RightParen,
    InfixToken::Operator(Operator::Add),
    InfixToken::Operand(3)];

    let y = None;
    assert_eq!(y, infix_to_postfix(x));

}
fn testcase23 (){
    //INFIX: (3+4)+4(3-4)
    //POSTFIX: ERROR
    let x = &[
    InfixToken::LeftParen,
    InfixToken::Operand(3),
    InfixToken::Operator(Operator::Add),
    InfixToken::Operand(4),
    InfixToken::RightParen,
    InfixToken::Operator(Operator::Add),
    InfixToken::Operand(4),
    InfixToken::LeftParen,
    InfixToken::Operand(3),
    InfixToken::Operator(Operator::Sub),
    InfixToken::Operand(4),
    InfixToken::RightParen
    ];

    let y = None;
    assert_eq!(y, infix_to_postfix(x));
}
fn testcase24 (){
    //INFIX: ()
    //POSTFIX: ERROR
    let x = &[
    InfixToken::LeftParen,
    InfixToken::RightParen
    ];

    let y = None;
    assert_eq!(y, infix_to_postfix(x));
}
fn testcase25 () {
    //INFIX: )(3+4
    //POSTFIX: ERROR
    let x = &[
    InfixToken::RightParen,
    InfixToken::LeftParen,
    InfixToken::Operand(3),
    InfixToken::Operator(Operator::Add),
    InfixToken::Operand(4)];

    let y = None;
    assert_eq!(y, infix_to_postfix(x));
}
fn testcase26 (){
    //INFIX: 3+4-
    //POSTFIX: ERROR
    let x = &[
    InfixToken::Operand(3),
    InfixToken::Operator(Operator::Add),
    InfixToken::Operand(4),
    InfixToken::Operator(Operator::Sub),];

    let y = None;
    assert_eq!(y, infix_to_postfix(x));
}
fn testcase27 (){
    //INFIX: (3+4)
    //POSTFIX: 3 4 +
    let x = &[
    InfixToken::LeftParen,
    InfixToken::Operand(3),
    InfixToken::Operator(Operator::Add),
    InfixToken::Operand(4),
    InfixToken::RightParen];

    let y = None;
    assert_eq!(y, infix_to_postfix(x));
}
fn testcase 28(){
    //INFIX: (3+4)() //Test case 13 & 19 on gradebot :D
    //POSTFIX: ERROR
    let x = &[
    InfixToken::LeftParen,
    InfixToken::Operand(3),
    InfixToken::Operator(Operator::Add),
    InfixToken::Operand(4),
    InfixToken::RightParen,
    InfixToken::LeftParen,
    InfixToken::RightParen];

    let y = None;
    assert_eq!(y, infix_to_postfix(x));
}
