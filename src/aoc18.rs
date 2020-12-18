#[derive(Debug, PartialEq, Eq)]
enum Token {
    Num(u64),
    OpMul,
    OpAdd,
    LParen,
    RParen,
}

fn eval1(tokens: &[Token]) -> u64 {
    let mut stk = Vec::with_capacity(16);
    for tok in tokens {
        match tok {
            Token::OpAdd => stk.push(Token::OpAdd),
            Token::OpMul => stk.push(Token::OpMul),
            Token::LParen => stk.push(Token::LParen),
            Token::Num(x) => {
                if let Some(last) = stk.last() {
                    if Token::OpMul == *last || Token::OpAdd == *last {
                        let op = stk.pop().unwrap();
                        if let Some(Token::Num(y)) = stk.pop() {
                            let result = match op {
                                Token::OpAdd => y + x,
                                Token::OpMul => y * x,
                                _ => unreachable!(),
                            };
                            stk.push(Token::Num(result));
                        } else {
                            panic!("could not pop number");
                        }
                    } else {
                        stk.push(Token::Num(*x));
                    }
                } else {
                    stk.push(Token::Num(*x));
                }
            }
            Token::RParen => {
                let result = stk.pop().unwrap();
                stk.pop(); //Lparen
                if let Token::Num(x) = result {
                    if let Some(last) = stk.last() {
                        if Token::OpMul == *last || Token::OpAdd == *last {
                            let op = stk.pop().unwrap();
                            if let Some(Token::Num(y)) = stk.pop() {
                                let result = match op {
                                    Token::OpAdd => y + x,
                                    Token::OpMul => y * x,
                                    _ => unreachable!(),
                                };
                                stk.push(Token::Num(result));
                            } else {
                                panic!("could not pop number");
                            }
                        } else {
                            stk.push(result);
                        }
                    } else {
                        stk.push(result);
                    }
                }
            }
        }
    }
    if let Token::Num(x) = stk[0] {
        x
    } else {
        unreachable!();
    }
}

fn eval2(tokens: &[Token]) -> u64 {
    let mut stk = Vec::with_capacity(tokens.len() * 2);
    stk.push(Token::LParen);
    for tok in tokens {
        match tok {
            Token::OpAdd => stk.push(Token::OpAdd),
            Token::OpMul => {
                stk.push(Token::RParen);
                stk.push(Token::OpMul);
                stk.push(Token::LParen);
            }
            Token::LParen => {
                stk.push(Token::LParen);
                stk.push(Token::LParen);
            }
            Token::Num(x) => {
                stk.push(Token::Num(*x));
            }
            Token::RParen => {
                stk.push(Token::RParen);
                stk.push(Token::RParen);
            }
        }
    }
    stk.push(Token::RParen);
    eval1(&stk)
}

pub(crate) fn run(data: &[u8]) -> String {
    let mut p = 0;
    let mut p1 = 0;
    let mut p2 = 0;
    let mut stk = Vec::new();

    while p < data.len() {
        //dbg!(&stk);
        //dbg!(data[p] as char);
        match data[p] {
            b' ' => {}
            x if x >= b'0' && x <= b'9' => stk.push(Token::Num((x - b'0') as u64)),
            b'(' => stk.push(Token::LParen),
            b')' => stk.push(Token::RParen),
            b'*' => stk.push(Token::OpMul),
            b'+' => stk.push(Token::OpAdd),
            b'\n' => {
                let r1 = eval1(&stk);
                //dbg!(r1);
                p1 += r1;
                let r2 = eval2(&stk);
                //dbg!(r2);
                p2 += r2;
                stk.clear();
            }
            _ => unreachable!(data[p] as char),
        }
        p += 1;
        /*match data[p] {
            x if x >= b'0' && x <= b'9' => {
                let v = (x - b'0') as u64;
                if let Some(Token::Op(o)) = stk.last() {
                    let op = *o;
                    stk.pop();
                    if let Some(Token::Num(y)) = stk.pop() {
                        let result = match op {
                            b'+' => y + v,
                            // b'-' => y - v,
                            b'*' => y * v,
                            //b'/' => y / v,
                            _ => unreachable!(op as char),
                        };
                        stk.push(Token::Num(result));
                    } else {
                        panic!("could not pop number");
                    }
                } else {
                    stk.push(Token::Num(v));
                }
            }
            b'(' => stk.push(Token::LParen),
            b')' => {
                let result = stk.pop().unwrap();
                stk.pop(); //Lparen
                if let Token::Num(x) = result {
                    if let Some(Token::Op(o)) = stk.last() {
                        let op = *o;
                        stk.pop();
                        if let Some(Token::Num(y)) = stk.pop() {
                            let result = match op {
                                b'+' => y + x,
                                //b'-' => y - x,
                                b'*' => y * x,
                                //b'/' => y / x,
                                _ => unreachable!(op as char),
                            };
                            stk.push(Token::Num(result));
                        } else {
                            panic!("could not pop number");
                        }
                    } else {
                        stk.push(result);
                    }
                } else {
                    stk.push(result);
                }
            }
            b' ' => {}
            x @ b'*' | x @ b'+' | x @ b'-' | x @ b'/' => {
                stk.push(Token::Op(x));
            }
            b'\n' => {
                dbg!(&stk);
                let result = stk.pop().unwrap();
                assert_eq!(stk.len(), 0);
                if let Token::Num(x) = result {
                    dbg!(x);
                    p1 += x;
                } else {
                    unreachable!();
                }
            }
            _ => unreachable!(data[p] as char),
        }
        p += 1;*/
    }

    format!("{} {}\n", p1, p2)
}
