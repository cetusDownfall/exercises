extern crate util;
use std::thread;
use std::sync::mpsc;
use std::cmp::Ordering;
use util::wolf;

fn main() {
    let mut gen = wolf::WolfGen::new();
    let mut i = 255;
    let mut t = Vec::new();
    while i > 0 {
        i -= 1;
        let num = gen.next_u8();
        t.push(num);
    }
    for i in &t {
        print!("{} ", i);
    }
    println!("\n");
    println!("Sorted");
    let f = split_to_threads(t);
    for i in &f {
        print!("{:?} ", i);
    }
    struct VecPair<T: Ord>(Vec<T>, Vec<T>);
    impl<T: Ord> VecPair<T> {
        fn merge(mut self) -> Vec<T> {
            let mut out: Vec<T> = Vec::new();
            loop { match (self.0.pop(), self.1.pop()) {
                    (None, None)        =>  break,
                    (Some(v), None) | (None, Some(v))  =>  { out.push(v); }, 
                    (Some(l), Some(r))  =>  
                        match r.cmp(&l) {
                            Ordering::Less      =>  { out.push(l); self.1.push(r); },
                            Ordering::Equal     =>  { out.push(l); out.push(r); },
                            Ordering::Greater   =>  { out.push(r); self.0.push(l); },
                        },}}
            out.reverse(); out 
        }
    }
    fn sort_in_thread<T: Ord>(mut target: Vec<T>) -> Option<Vec<T>> {
        if target.len() > 1 {
            let l = target.len()/2;
            let r = target.split_off(l);
            let mut p = VecPair(target, r);
            p = VecPair(match sort_in_thread(p.0) { Some(v) => v, None => vec![], }, match sort_in_thread(p.1) { Some(v) => v, None => vec![], });
            Some(p.merge())
        } else if target.len() == 1 {
            Some(target)
        } else { None }
    }
    fn split_to_threads<T: 'static + Ord + Send + Clone>(mut target: Vec<T>) -> Vec<T> {
        let (tx, rx) = mpsc::channel();
        let mut thread_ct = 1;
        while thread_ct * thread_ct < target.len() { thread_ct *= 2; }
        let mut split_targ = vec![Vec::new(); thread_ct];
        let mut i = 0;
        while let Some(n) = target.pop() {
            split_targ[i].push(n);
            i = (i+1)%thread_ct;
        }
        for i in split_targ {
            let tx = mpsc::Sender::clone(&tx);
            thread::spawn(move || {
                tx.send(sort_in_thread(i)).unwrap();
            });
        }
        let mut msg_ct = 0;
        let mut finished_threads = Vec::new();
        while msg_ct < thread_ct {
            if let Ok(v) = rx.try_recv() {
                if let Some(n) = v {
                    finished_threads.push(n);
                    msg_ct += 1;
                }
            }
            if msg_ct%2 == 0 {
                match (finished_threads.pop(), finished_threads.pop()) {
                    (Some(l), Some(r))  =>  finished_threads.insert(0, VecPair(l,r).merge()),
                    (Some(v), None)     =>  finished_threads.insert(0, v),
                    (None,  _     )     =>  (),
                }
            }
        }
        while finished_threads.len() > 1 {
            if let (Some(l), Some(r)) = (finished_threads.pop(), finished_threads.pop()) { finished_threads.push(VecPair(l,r).merge()); }
        }
        match finished_threads.pop() {
            Some(n) => n,
            None    => vec![],
        }
    }
}
