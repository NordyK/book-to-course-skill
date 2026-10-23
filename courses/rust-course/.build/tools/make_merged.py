import json,os,sys
S=sys.argv[1]; out=os.path.join(S,'merged'); os.makedirs(out+'/chapters',exist_ok=True)
B={'trpl':'books/trpl-2024/','pr':'books/programming-rust-2e/chapters/','ria':'books/rust-in-action/chapters/','cr':'books/comprehensive-rust/chapters/'}
CH=[
("ch01","Getting Started with Rust",[("trpl","trpl00.md","Introduction"),("trpl","trpl01.md","Getting Started"),("pr","ch03-1-systems-programmers-can-have-nice-things.md","Ch1 Systems Programmers Can Have Nice Things"),("ria","ch04-chapter-1-introducing-rust.md","Ch1 Introducing Rust"),("cr","ch04-using-cargo.md","Using Cargo")]),
("ch02","Your First Program: a Guessing Game",[("trpl","trpl02.md","Programming a Guessing Game"),("pr","ch04-2-a-tour-of-rust.md","Ch2 A Tour of Rust")]),
("ch03","Common Programming Concepts",[("trpl","trpl03.md","Common Programming Concepts"),("pr","ch05-3-fundamental-types.md","Ch3 Fundamental Types"),("pr","ch08-6-expressions.md","Ch6 Expressions"),("ria","ch05-chapter-2-language-foundations.md","Ch2 Language Foundations"),("cr","ch05-i-day-1-morning.md","Day 1 Morning")]),
("ch04","Ownership, Borrowing and Slices",[("trpl","trpl04.md","Understanding Ownership"),("pr","ch06-4-ownership-and-moves.md","Ch4 Ownership and Moves"),("pr","ch07-5-references.md","Ch5 References"),("ria","ch07-chapter-4-lifetimes-ownership-and-borrowing.md","Ch4 Lifetimes, Ownership and Borrowing"),("cr","ch09-v-day-3-morning.md","Day 3 Morning (memory)")]),
("ch05","Structs: Grouping Related Data",[("trpl","trpl05.md","Using Structs"),("pr","ch11-9-structs.md","Ch9 Structs"),("ria","ch06-chapter-3-compound-data-types.md","Ch3 Compound Data Types"),("cr","ch06-ii-day-1-afternoon.md","Day 1 Afternoon")]),
("ch06","Enums and Pattern Matching",[("trpl","trpl06.md","Enums and Pattern Matching"),("pr","ch12-10-enums-and-patterns.md","Ch10 Enums and Patterns"),("cr","ch07-iii-day-2-morning.md","Day 2 Morning")]),
("ch07","Packages, Crates and Modules",[("trpl","trpl07.md","Packages, Crates, and Modules"),("pr","ch10-8-crates-and-modules.md","Ch8 Crates and Modules"),("cr","ch11-vii-day-4-morning.md","Day 4 Morning (modules)")]),
("ch08","Common Collections and Text",[("trpl","trpl08.md","Common Collections"),("pr","ch18-16-collections.md","Ch16 Collections"),("pr","ch19-17-strings-and-text.md","Ch17 Strings and Text"),("cr","ch08-iv-day-2-afternoon.md","Day 2 Afternoon (std)")]),
("ch09","Error Handling",[("trpl","trpl09.md","Error Handling"),("pr","ch09-7-error-handling.md","Ch7 Error Handling"),("cr","ch12-viii-day-4-afternoon.md","Day 4 Afternoon (errors)")]),
("ch10","Generics, Traits and Lifetimes",[("trpl","trpl10.md","Generic Types, Traits, and Lifetimes"),("pr","ch13-11-traits-and-generics.md","Ch11 Traits and Generics"),("pr","ch14-12-operator-overloading.md","Ch12 Operator Overloading"),("pr","ch15-13-utility-traits.md","Ch13 Utility Traits"),("cr","ch07-iii-day-2-morning.md","Day 2 Morning (traits, generics)")]),
("ch11","Writing Automated Tests",[("trpl","trpl11.md","Writing Automated Tests"),("cr","ch11-vii-day-4-morning.md","Day 4 Morning (testing)")]),
("ch12","Project: a Command-Line Search Tool",[("trpl","trpl12.md","An I/O Project"),("pr","ch20-18-input-and-output.md","Ch18 Input and Output")]),
("ch13","Closures and Iterators",[("trpl","trpl13.md","Iterators and Closures"),("pr","ch16-14-closures.md","Ch14 Closures"),("pr","ch17-15-iterators.md","Ch15 Iterators"),("cr","ch08-iv-day-2-afternoon.md","Day 2 Afternoon (closures, iterators)")]),
("ch14","Cargo and Crates.io",[("trpl","trpl14.md","More About Cargo and Crates.io")]),
("ch15","Smart Pointers",[("trpl","trpl15.md","Smart Pointers"),("ria","ch09-chapter-6-memory.md","Ch6 Memory"),("cr","ch09-v-day-3-morning.md","Day 3 Morning (Box, Rc)")]),
("ch16","Fearless Concurrency",[("trpl","trpl16.md","Fearless Concurrency"),("pr","ch21-19-concurrency.md","Ch19 Concurrency"),("ria","ch13-chapter-10-processes-threads-and-containers.md","Ch10 Processes, Threads and Containers"),("cr","ch17-xiii-concurrency-morning.md","Concurrency Morning")]),
("ch17","Async Programming",[("trpl","trpl17.md","Async, Await, Futures, Streams"),("pr","ch22-20-asynchronous-programming.md","Ch20 Asynchronous Programming"),("cr","ch18-xiv-concurrency-afternoon.md","Concurrency Afternoon (async)")]),
("ch18","Trait Objects and OOP in Rust",[("trpl","trpl18.md","Object-Oriented Programming Features"),("cr","ch10-vi-day-3-afternoon.md","Day 3 Afternoon")]),
("ch19","Patterns in Depth",[("trpl","trpl19.md","Patterns and Matching"),("pr","ch12-10-enums-and-patterns.md","Ch10 Enums and Patterns (patterns half)")]),
("ch20","Advanced Features: Unsafe, Advanced Traits and Types, Macros",[("trpl","trpl20.md","Advanced Features"),("pr","ch23-21-macros.md","Ch21 Macros"),("pr","ch24-22-unsafe-code.md","Ch22 Unsafe Code"),("cr","ch20-xvi-unsafe.md","Unsafe")]),
("ch21","Data in Depth: Bits, Integers and Floats",[("ria","ch08-chapter-5-data-in-depth.md","Ch5 Data in Depth"),("pr","ch05-3-fundamental-types.md","Ch3 Fundamental Types (representation)")]),
("ch22","Files, Storage and I/O",[("ria","ch10-chapter-7-files-storage.md","Ch7 Files & Storage"),("pr","ch20-18-input-and-output.md","Ch18 Input and Output")]),
("ch23","Networking and Time",[("ria","ch11-chapter-8-networking.md","Ch8 Networking"),("ria","ch12-chapter-9-time-and-time-keeping.md","Ch9 Time and Time Keeping")]),
("ch24","Going Low-Level: FFI, Kernels and Signals",[("pr","ch25-23-foreign-functions.md","Ch23 Foreign Functions"),("ria","ch14-chapter-11-kernel.md","Ch11 Kernel"),("ria","ch15-chapter-12-signals-interrupts-and-exceptions.md","Ch12 Signals, Interrupts and Exceptions"),("cr","ch15-xi-bare-metal-morning.md","Bare Metal")]),
("ch25","Final Project: a Multithreaded Web Server",[("trpl","trpl21.md","Final Project"),("cr","ch19-xv-idiomatic-rust.md","Idiomatic Rust")]),
]
NAMES={'trpl':'The Rust Programming Language (2024 ed., Rust 1.97)','pr':'Programming Rust, 2nd ed.','ria':'Rust in Action','cr':'Comprehensive Rust'}
man={'title':'Rust: From First Program to Systems Programming','author':'Merged from 5 books (see .build/plan.md)','language':'en','source':'merged','format':'merged','chapters':[]}
for cid,title,srcs in CH:
    lines=['# '+title,'','Source files to read (relative to .build/source/):','']
    words=0
    for k,f,label in srcs:
        p=os.path.join(S,B[k]+f)
        w=len(open(p,encoding='utf-8').read().split()) if os.path.exists(p) else -1
        if w<0: print('MISSING',p)
        words+=max(w,0)
        lines.append('- %s — %s → `%s%s` (%d words)'%(NAMES[k],label,B[k],f,w))
    fn=cid+'.md'; open(os.path.join(out,'chapters',fn),'w').write('\n'.join(lines)+'\n')
    man['chapters'].append({'id':cid,'title':title,'words':words,'file':'chapters/'+fn,'kind':'chapter'})
json.dump(man,open(os.path.join(out,'manifest.json'),'w'),indent=1)
open(os.path.join(out,'outline.md'),'w').write('\n'.join('## %s — %s'%(c['id'],c['title']) for c in man['chapters']))
print('ok',len(CH))
