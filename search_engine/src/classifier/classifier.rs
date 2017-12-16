use std::fmt;
use std::cmp::Ordering;
use std::ops::Add;
use std::ops::Div;

pub trait Classifier<'a> {
    fn classify(&self, doc_id: u32) -> &'a str;
    fn get_all_vocab(&self) -> Vec<String>;
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum DocumentClass {
    Hamilton,
    Jay,
    Madison,
}

#[derive(Debug, PartialOrd)]
pub struct TermClassScore {
    pub score: f64,
    pub term: String,
    pub class: DocumentClass,
}

impl TermClassScore {
    pub fn new(score: f64, term: String, class: DocumentClass) -> Option<TermClassScore> {
        if score.is_nan() {
            println!("Looks like there was a NaN! Term is: {}. Class is: {:?}.", term, class);
            None
        }
        else {
            Some(TermClassScore {
                score: score,
                term: term,
                class: class,
           })
        }
    }
}

impl fmt::Display for TermClassScore {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Term: {}, Score: {}", self.term, self.score)
    }
}

impl Eq for TermClassScore {

}

impl Ord for TermClassScore {
    fn cmp(&self, other: &TermClassScore) -> Ordering {
        self.score.partial_cmp(&other.score).unwrap()
    }
}

impl PartialEq for TermClassScore {
    fn eq(&self, other: &TermClassScore) ->bool {
        self.term == other.term 
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Scalar {
    value: f64
}

impl Scalar {
    pub fn new(v: f64) -> Scalar {
        Scalar {
            value: v 
        }
    }
}


#[derive(Debug, Clone,  PartialOrd)]
pub struct TermComponentScore {
    pub score: f64,
    pub term: String,
}

impl TermComponentScore {
    pub fn new(score: f64, term: String) -> Option<TermComponentScore> {
        if score.is_nan() {
            println!("Looks like there was a NaN! Term is: {}", term);
            None
        }
        else {
            Some(TermComponentScore {
                score: score,
                term: term,
           })
        }
    }
}

impl fmt::Display for TermComponentScore {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Term: {}, Score: {}", self.term, self.score)
    }
}

impl Eq for TermComponentScore {

}

impl Ord for TermComponentScore {
    fn cmp(&self, other: &TermComponentScore) -> Ordering {
        self.term.partial_cmp(&other.term).unwrap()
    }
}

impl PartialEq for TermComponentScore {
    fn eq(&self, other: &TermComponentScore) ->bool {
        self.term == other.term 
    }
}

impl Add for TermComponentScore {
    type Output = TermComponentScore;

    fn add(self, other: TermComponentScore) -> TermComponentScore {
        TermComponentScore {
            term: self.term, 
            score: self.score + other.score,     
        }
    }

}

impl Div<Scalar> for TermComponentScore {
    type Output = TermComponentScore;

    fn div (self, RHS: Scalar) -> TermComponentScore {
        TermComponentScore {
            term: self.term,
            score: self.score / RHS.value
        } 
    }
}

impl Div<Scalar> for Vec<TermComponentScore> {
    type Output = Vec<TermComponentScore>;

    fn div(self, RHS: Scalar) -> Vec<TermComponentScore> {
        let mut new_tcs : Vec<TermComponentScore> = Vec::new();

        let RHS_clone = RHS.clone();
        for tcs in self {
            new_tcs.push(tcs/RHS_clone);
        }
        new_tcs
    }

}



