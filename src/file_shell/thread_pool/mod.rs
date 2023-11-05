use std::{
    sync::{
        mpsc::{self, Receiver},
        Arc, Mutex,
    },
    thread::{self, JoinHandle, Thread},
};

pub struct ThreadPool {
    //构建线程池以处理并发
    workers: Vec<Worker>, //线程池本身并不执行线程，执行线程的是Worker
    sender: Option<mpsc::Sender<Job>>, //sender用于发送任务给worker
}
type Job = Box<dyn FnOnce() + Send + 'static>; //类型别名
impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        //使用消息管道创建发送者和接收者
        let (sender, receiver) = mpsc::channel();
        //给接收者上锁，因为接收者只有一个，而多个worker需要使用接收者
        let receiver = Arc::new(Mutex::new(receiver));
        //创建worker数组
        let mut workers: Vec<Worker> = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool {
            workers: workers,
            sender: Some(sender),
        }
    }

    ///用于执行任务
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        //把任务放入堆中（因为任务不是定长的）
        let job = Box::new(f);
        //把任务的指针发送给worker，由于send函数是阻塞函数，所以当线程池满了的时候，程序会自动阻塞
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            // println!("Shutting down worker {}",worker.id);
            //等待take函数是拿走worker的接收者，这样子worker就不会再接收新任务了
            if let Some(thread) = worker.thread.take() {
                //等待worker线程的结束，等所有worker线程结束后，线程池就可以回收了
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        //每个worker的线程都是一个无限循环，它会监听接收者，如果没有任务发过来就阻塞，如果有任务就执行
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    job();
                }
                Err(_) => {
                    break;
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}
