import re,sys,os,json
src,out=sys.argv[1],sys.argv[2]
lines=open(src,encoding='utf-8').read().split('\n')
noise=re.compile(r'^\s*(\d+/\d+/\d+, \d+:\d+ [AP]M\s+The Rust Programming Language|https://doc\.rust-lang\.org/book/print\.html\s+\d+/\d+)\s*$')
pg=re.compile(r'^<!-- page \d+ -->$|^# Part \d+')
starts=[(160,'trpl01','Getting Started'),(619,'trpl02','Programming a Guessing Game'),(1347,'trpl03','Common Programming Concepts'),(2800,'trpl04','Understanding Ownership'),(4086,'trpl05','Using Structs'),(4898,'trpl06','Enums and Pattern Matching'),(5686,'trpl07','Packages, Crates, and Modules'),(6679,'trpl08','Common Collections'),(7498,'trpl09','Error Handling'),(8248,'trpl10','Generic Types, Traits, and Lifetimes'),(9739,'trpl11','Writing Automated Tests'),(11112,'trpl12','An I/O Project'),(12429,'trpl13','Iterators and Closures'),(13550,'trpl14','More About Cargo and Crates.io'),(14313,'trpl15','Smart Pointers'),(16013,'trpl16','Fearless Concurrency'),(16988,'trpl17','Async, Await, Futures, Streams'),(18615,'trpl18','Object-Oriented Programming Features'),(19543,'trpl19','Patterns and Matching'),(20575,'trpl20','Advanced Features'),(22461,'trpl21','Final Project: Multithreaded Web Server'),(24206,'trpl-appendix','Appendices'),(25135,None,None)]
starts.insert(0,(47,'trpl00','Introduction'))
os.makedirs(out,exist_ok=True)
man=[]
for (a,i,t),(b,_,_) in zip(starts,starts[1:]):
    body=[l for l in lines[a-1:b-1] if not noise.match(l) and not pg.match(l)]
    txt='\n'.join(body)
    fn=i+'.md'
    open(os.path.join(out,fn),'w',encoding='utf-8').write('# '+t+'\n\n'+txt)
    man.append({'id':i,'title':t,'file':fn,'words':len(txt.split())})
json.dump(man,open(os.path.join(out,'index.json'),'w'),indent=1)
for m in man: print(m['id'],m['words'],m['title'])
