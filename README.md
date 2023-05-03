# mumps web resources

### Quick Reference
https://github.com/mumps-dev/mumps-dev.github.io/tree/main/quick-ref  


### MUMPS Tutorials - Prof O'Kane
https://www.cs.uni.edu/~okane/index.html  
https://www.cs.uni.edu/~okane/mumps.html  
https://www.cs.uni.edu/~okane/youtube.html#mumps  
https://youtube.com/playlist?list=PLmMgHNtOIstYpwQRy5btu-P-Gl90IsjPM  


### Hacker News
Introduction to the Mumps Language  
https://news.ycombinator.com/item?id=16309237  

```text
	Introduction to the Mumps Language (2017) [pdf] (uni.edu)
	67 points by twunde on Feb 5, 2018 | hide | past | favorite | 42 comments


	
	
mxtbccagmailcom on Feb 6, 2018 | next [–]

My first professional development language. It was old, when I started to use it in 1986. You had to be disciplined - 'cus' writing spaghetti was way too easy. My style was "structured goto's". :-) You got to remember this language was written on a PDP-7 with 4k -that's 4096 bytes- of main memory and before Edsger Dijkstra's letter "Go To Statement Considered Harmful", published in March 1968.

Wrote a simple web server with it for a VAX in 1999.

Loved how you saved data to disk. You just but a ^ in front of a variable declaration:

   ^myData

myData could be a "hash map" by doing an assignment with parameters

   set ^myData("canada", "bc", "yyj") = counter

Thinking about it there's not a whole lot of distance between it and javascript. Guess that's why I like JS so much!

	
	
epmatsw on Feb 6, 2018 | prev | next [–]

Mumps! There's a couple of docker images floating around out there for playing around in it. If anyone hasn't tried it, it's "fun". M's global data stores, weird scoping, and strange syntax make for some interesting problems :P

https://hub.docker.com/r/tsafin/fis-gtm-env/

https://github.com/KRMAssociatesInc/docker-vista

	
	
nwhatt on Feb 6, 2018 | parent | next [–]

Time to relive my Epic glory days. Seriously thanks for sharing these and appreciate your positive attitude.

	
	
epmatsw on Feb 6, 2018 | root | parent | next [–]

I’m also an ex-Epic guy! It’s been a while since I played with these (M without EpicStudio is painful), but I think starting my career in M definitely helped me as a developer. Every time I hear someone complain about JS at work I’m like “If you think that’s crazy...”

	
	
lgessler on Feb 6, 2018 | prev | next [–]

A lot of this language's unreadability is due to how it was originally interpreted instead of compiled because of the cost of memory at the time of its invention:

> Since memory was tight originally, the language design for MUMPS valued very terse code. Thus, every MUMPS command or function name could be abbreviated from one to three letters in length, e.g. Quit (exit program) as Q, $P = $Piece function, R = Read command, $TR = $Translate function. Spaces and end-of-line markers are significant in MUMPS because line scope promoted the same terse language design. Thus, a single line of program code could express, with few characters, an idea for which other programming languages could require 5 to 10 times as many characters. Abbreviation was a common feature of languages designed in this period (e.g., FOCAL-69, early BASICs such as Tiny BASIC, etc.). An unfortunate side effect of this, coupled with the early need to write minimalist code, was that MUMPS programmers routinely did not comment code and used extensive abbreviations. This meant that even an expert MUMPS programmer could not just skim through a page of code to see its function but would have to analyze it line by line.

https://en.wikipedia.org/wiki/MUMPS#Overview

I'll add that Wikipedia's take here is a little optimistic: perhaps it was true when MUMPS was invented that it could use one line to accomplish what would require "5 to 10 times as many characters" in other languages, but MUMPS's language constructs are by and large about as low level as assembly with a couple exceptions, so this is definitely no longer the case if we're comparing it to modern high level languages.

In addition to Epic, the US Department of Veterans Affairs wrote their homegrown EMR in MUMPS. Their development group is here: https://groups.google.com/forum/#!forum/hardhats

	
	
telotortium on Feb 6, 2018 | parent | next [–]

> I'll add that Wikipedia's take here is a little optimistic: perhaps it was true when MUMPS was invented that it could use one line to accomplish what would require "5 to 10 times as many characters" in other languages, but MUMPS's language constructs are by and large about as low level as assembly with a couple exceptions, so this is definitely no longer the case if we're comparing it to modern high level languages.

Eh, as someone who's used it professionally, I wouldn't compare it to assembly. It's more like Awk - fairly concise for a good deal of its intended uses (linewise text manipulation for Awk, trees for MUMPS) and horribly clunky for most everything else.

	
	
ptx on Feb 6, 2018 | parent | prev | next [–]

Also mentioned in the linked tutorial: Memory on early mini-computers was limited, sometimes only a few thousand characters. Mumps programs were loaded into memory as source code rather than as compiled binary. This was done because it was determined that compiled Mumps programs would be far larger than the corresponding source code versions, especially if the source code employed size reducing abbreviations.

So why didn't they use a bytecode interpreter, or at least a minifier (as is done with JavaScript), so that the programmer could keep the source code readable while still giving the interpreter a compact version?

Would it have been too computationally expensive, or had people just not come up with the idea yet?

	
	
hyperpape on Feb 6, 2018 | root | parent | next [–]

I’m not at all sure, but poking around on Wikipedia suggests that the idea of bytecode interpreters might have dated to roughly the same time as MUMPS, which means it wouldn’t have necessarily been the obvious choice at that time. https://en.m.wikipedia.org/wiki/P-code_machine

	
	
dahakon on Feb 6, 2018 | root | parent | prev | next [–]

InterSystems Caché (one of the most popular Mumps versions today because InterSystems bought out a lot of the market) uses a bytecode interpreter. Your source goes in a ^ROUTINE global and the bytecode gets sent to a ^rOBJ global. And yes, the bytecode is usually much smaller since it is stripped of comments (and has other size optimizations).

	
	
epmatsw on Feb 6, 2018 | root | parent | prev | next [–]

I’m going to go with they hadn’t come up with it yet. Modern M, at least at Epic, goes through a few levels of transpilation/compilation before ending up running in Java (I think that’s what Cache is now). New code isn’t really much worse than any other codebase IMO, but some of the old code there is crazy.

	
	
dahakon on Feb 6, 2018 | root | parent | next [–]

Some companies use their own higher-level layer on top of M. For example, FIS (banking software) uses Profile Scripting language which compiles to M which compiles to native machine code. Cache compiles to custom bytecode unrelated to Java or the Java Virtual Machine.

	
	
flyinghamster on Feb 6, 2018 | parent | prev | next [–]

>> Since memory was tight originally, the language design for MUMPS valued very terse code. Thus, every MUMPS command or function name could be abbreviated from one to three letters in length, e.g. Quit (exit program) as Q, $P = $Piece function, R = Read command, $TR = $Translate function. Spaces and end-of-line markers are significant in MUMPS because line scope promoted the same terse language design.

That makes me think of TECO, where a string of commands resembled transmission line noise more than anything resembling program code.

The most TECO that I ever learned was how to invoke VTEDIT, and I forgot that a long, long time ago. :)

	
	
killjoywashere on Feb 6, 2018 | prev | next [–]

I am entering data and running queries on a MUMPS system right now, over SSH running OpenVMS 8.4. Last application software update: 1999.

I suspect we would keep using it, except, last I heard, HP off-shored their OpenVMS maintanance to India and declared, in like 2011, that it would be EOL'd in 2017.

edit: I retract the statement that it's running on a PDP-11, though I have to say I was told that in teleconference just last month. I remember because it was shocking.

	
	
xeeeeeeeeeeenu on Feb 6, 2018 | parent | next [–]

1. OpenVMS doesn't run on PDP-11. The only platforms it has ever supported are VAX, Alpha and Itanium.

2. OpenVMS is alive and well. It wasn't outsourced to India and it does not have EOL date. Currently it's being developed by VSI (https://www.vmssoftware.com/) and they're porting it to x86.

	
	
killjoywashere on Feb 6, 2018 | root | parent | next [–]

perhaps I misrepresented "MPS w/o SE: Mature Product Support without Sustaining Engineering"

http://h41379.www4.hpe.com/openvms/openvms_supportchart.html

	
	
xeeeeeeeeeeenu on Feb 6, 2018 | root | parent | next [–]

This date applies only to OpenVMS 8.4. New releases are planned, for example, 8.5 will be released this year. See the current roadmap: https://www.vmssoftware.com/pdfs/VSI_Roadmap_20171215.pdf

	
	
moarider on Feb 6, 2018 | prev | next [–]

Epic Systems (the biggest vendor of EMRs) is one of the primary users of Mumps/Caché. In the early days of the company, technical support was allowed to write in customizations for clients as long as they could fit in a single line of code. This was possible given that one 'feature' of the language is that all the reserved words have a one letter alias. So if you were one of the unlucky ones assigned to support these old customers, you'd run across stuff that looked like "s a w; a f 10; a f s w ^a;".

	
	
emersonrsantos on Feb 6, 2018 | prev | next [–]

One company have a object-oriented database programmed under Mumps that is so backwards compatible to the point it can interface to RS232 dumb terminals and run utilites and code from the 70's. Used very much by governments and hospitals, it's one of the most insecure platforms I've ever worked.

Bonus - perfectly valid Mumps code

  P       R I N T S (A,L)=1,(S,T)=I N G
          G I V E N A
  O       F A=L:L:S Q:U=A R E  S B=E L O W (A*A),!
  I       S (U,R,E)=T H 1 N K I T=S G O O D

	
	
DennisP on Feb 6, 2018 | prev | next [–]

Several years ago the VA announced a public contest to update their MUMPS-based system, with a couple million dollars in prize money for good proofs of concept. The system is open source so I downloaded it for a look. Half an hour later I closed my text windows with a sigh and gave up on that idea.

But I can't say for sure that it's entirely the fault of MUMPS rather than the amateurs who wrote the code. It literally was amateurs; they were doctors and interns, not professional programmers. They made a system that fit their needs quite well at the time, so they deserve quite a bit of credit, even though their source code is a nightmare by modern standards. Their story is recounted in the book Best Care Anywhere.

https://www.amazon.com/dp/B0073HW03I/

	
	
gumby on Feb 6, 2018 | prev | next [–]

Little known is that one of the major MUMPS implementors, Harlan Stenn, is also the NTP maintainer. I remember him working on GNU MUMPS in the mid 90s, though it doesn’t look like thenimplementation still on his website is GPLed.

	
	
tekkk on Feb 6, 2018 | prev | next [–]

The new finnish medical system I heard is being built with Mumps Accenture being the main contractor. Fun times, I imagine.

Total cost approx. 2 billion euros majority going to Accenture and Epic Systems (which if you didn't read thedailywtf recommend checking out). The new medical system in Estonia cost about 5 millions I heard. It's just incomprehensible to me how they are going spend 400 times more money with 5x more population.

	
	
twunde on Feb 6, 2018 | parent | next [–]

Epic is famous for being SUPER-expensive. One of my colleagues told me a story about how some place build a new hospital for I think $500Million and then the Epic installation was $600M. Granted, I think Epic is the most expensive of the EMRs, but it's crazy that Epic is price-effective in ANY way. Surprisingly it is for large hospitals. It's actually more cost-effective to use Epic than to use Athena

	
	
nwhatt on Feb 6, 2018 | prev | next [–]

This was posted last year.

https://news.ycombinator.com/item?id=13859961

Does anyone know the story behind why these slides keep getting updated? Is this a databases class?

	
	
twunde on Feb 6, 2018 | parent | next [–]

OP here. Looking at the author's CV (http://www.cs.uni.edu/~okane/resume.html), he's apparently written a book on Mumps revised in 2016 and presumably does some consulting as well. I would imagine that he incorporates it into at least one of the classes he teaches.

	
	
nwhatt on Feb 19, 2018 | root | parent | next [–]

He's apparently also a conservative fiction writer: http://threadsafebooks.com/?page_id=103

> Caution: Not recommended for moonbats, liberals, Clinton drones or Bernie’s Sandernistas.

	
	
jerrysievert on Feb 5, 2018 | prev | next [–]

in an different life, I was responsible for maintaining a MUMPS application that had been built by a contractor as a SaaS app. it took until the first set of feature requests before I made the decision that it needed to be rewritten in a language that allowed for tests and maintainability before any additional features could be added (not a decision made lightly). to this day, I am shocked that so much medical software is written in this language.

as an aside, there were some positive notes, as it was my first real introduction to a noSQL database (Caché) in a professional setting, 2006.

	
	
yodelsnorter on Feb 6, 2018 | prev | next [–]

Of the places I've encountered Mumps/Pick/Cache, there has always been that one gatekeeper guy - only he knew the arcane abbreviations and structure in his head, everyone else had to bow and scrape to get things done.

Very good language if unmaintainable, arcane code and/or job security through obscurity is your thing. Cache even let it onto the web some years back I think with ObjectScript. Just wholly nasty. The politics in medical software has been pretty rife from what I've experienced too.

	
	
ams6110 on Feb 6, 2018 | parent | next [–]

I remember full page ads for Cache in Datamation and similar IT-management-oriented publications, in the mid 1990s. They called it a "post-relational database" without ever really explaining what that meant.

	
	
sedachv on Feb 6, 2018 | root | parent | next [–]

InterSystems switched to advertising Caché as an "object" database sometime in the 2000s (at least that is when I first saw their ads, maybe it started earlier). I assumed it was something like ObjectStore. Definitely felt deceived when I found out what it really was.

	
	
Upvoter33 on Feb 6, 2018 | prev | next [–]

mumps is well named - it's more like a disease you catch rather than a tool you'd want to use.

	
	
slipstream- on Feb 6, 2018 | prev | next [–]

This reminded me to check thedailywtf.

	
	
deepthought42 on Feb 6, 2018 | prev | next [–]

I had the distinct displeasure of working in a number of mumps derivatives for my first professional programming job. It's way too easy to write terrible code and even easier to write something that appears to work, but actually doesn't. What's even more terrifying is that these archaic languages still power a significant number of hospitals here in America. Hospitals haven't felt safe ever since.

	
	
jklein11 on Feb 6, 2018 | prev | next [–]

I have been workign with MuMPS(Cache Objectscript) for the last 3 years. I'm working on writing the backend of a web application with it currently. In my experience the first 3 or so months are a little painful because the syntax is a little bizarre and the documentation is a little bit cumbersome. At the end of the day code is just code and its not all that bad.

	
	
hprotagonist on Feb 5, 2018 | prev | next [–]

a pdf of a powerpoint deck is about what i'd expect to see here, actually!

I know one or two MUMPs people. They're deeply, deeply odd.

	
	
jimrandomh on Feb 6, 2018 | prev | next [–]

Since this is presented without context, and there's a significant chance people will get the wrong idea, I'm going to say this directly:

The Mumps programming language is widely used in the medical field, but it has a justified bad reputation. If you choose the Mumps programming language for a new project (as opposed to maintenance of an existing project where you don't get to choose the language), this is professional malpractice, and that goes double if the project is in any way safety-critical.

You have been warned.

	
	
Fins on Feb 6, 2018 | parent | next [–]

I'm certainly glad that I didn't have to touch any M in ages and ages, but there's a good chance that your medical records, your financial records, and possibly your telecom billing is stored with a good dose of MUMPS.

And I heard people do use NoSQL for "important" data, and NoSQL is basically a reinvention of M as a square wheel and with no good parts of it.

	
	
cordite on Feb 6, 2018 | parent | prev | next [–]

Had experience with it.

I'll give my goods and bads after about 3 years.

Good: 1. Its behavior is very predictable, both in performance and for conditional behavior. 2. Error handling is not leaky, which helps with predictability. 3. Memory sharing is pretty isolated when it comes to processes / threads, much like erlang

Bads: 1. If you want to do any sort of polymorphism or code reuse, consider yourself screwed. 2. It feels like assembly, everything ends up being tacked together and tangled for code reuse 3. What you write, and what you edit, and what you depend on will be long lived; version control blame will show code still running in production from before you even finished elementary school. Refactoring is impractical. 4. Event driven designs are nearly impossible to do in this environment. 5. Local environments are not a thing. You code in the same mainframe everyone else does. Good luck.

	
	
steelbird on Feb 6, 2018 | prev | next [–]

It's gotta have its niche else it wouldn't still exist, right? Medical records? I think...

	
	
MetaMonk on Feb 6, 2018 | prev | next [–]

I sat near Neil (MUMPS creator) at my first job. He definitely loves this creation of his.

	
	
calebm on Feb 6, 2018 | prev | next [–]

Someone please write antivirus software in this language.

	
	
WorldMaker on Feb 6, 2018 | parent | next [–]

I think the argument is that someone needs to distribute more vaccines against this language.

	
	
iwre0 on Feb 6, 2018 | prev [–]

I work with it now, and I like it ....

```
