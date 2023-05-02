2-Jul-96, 6:54:35
Routine Save for Trigonometry Library function $%ARCTAN^MATH
 ;
 ; Unless otherwise noted, the code below
 ; was approved in document X11/95-12
 ;
 ; If corrections have been applied,
 ; first the original line appears,
 ; with three semicolons at the beginning of the line.
 ;
 ; Then the source of the correction is acknowledged,
 ; then the corrected line appears, followed by a
 ; line containing three semicolons.
 ;
ARCTAN(X,PREC) ;
 New FOLD,HI,L,LIM,LO,K,SIGN,SIGS,SIGT,VALUE
 Set PREC=$Get(PREC,11)
 Set LO=0.0000000001,HI=9999999999
 Set SIGT=$Select(X<0:-1,1:1),X=$Translate(X,"-")
 Set X=$Select(X<LO:LO,X>HI:HI,1:X)
 ;
 ;;; Set FOLD=$Select(X'<1:0,1:1),
 ; Eli Reidler (28 June 1996)
 Set FOLD=$Select(X'<1:0,1:1)
 ;;;
 ;
 Set X=$Select(FOLD:1/X,1:X)
 Set L=X,VALUE=$%PI^MATH()/2-(1/X),SIGN=1
 ;
 ;;; If X<1.3 Do  Quit
 ; Winfried Gerum (8 June 1995)
 ; Eli Reidler (28 June 1996)
 If X<1.3 Do  Quit VALUE
 . ;;;
 . ;
 . Set X=$Select(FOLD:1/X,1:X),VALUE=1/((1/X/X)+1)
 . ;
 . ;;; Set $%SQRT^MATH(VALUE)
 . ; Winfried Gerum (8 June 1995)
 . ; Eli Reidler (28 June 1996)
 . Set X=$%SQRT^MATH(VALUE,PREC)
 . ;;;
 . ;
 . If $Translate(X,"-")=1 Do  Quit
 . . Set VALUE=$%PI^MATH()/2*X
 . . Quit
 . If X>0.9 Do  Quit
 . . Set SIGS=$Select(X<0:-1,1:1)
 . . Set VALUE=1/(1/X/X-1)
 . . Set X=$%SQRT^MATH(VALUE)
 . . Set VALUE=$$ARCTAN(X,10)
 . . Set VALUE=VALUE*SIGS
 . . Quit
 . Set (VALUE,L)=X
 . Set LIM=$Select((PREC+3)'>11:PREC+3,1:11),@("LIM=1E-"_LIM)
 . For K=3:2 Do  Quit:($Translate(L,"-")<LIM)
 . . Set L=L*X*X*(K-2)/(K-1)*(K-2)/K,VALUE=VALUE+L
 . . Quit
 . Set VALUE=$Select(SIGT<1:-VALUE,1:VALUE)
 . Quit
 Set LIM=$Select((PREC+3)'>11:PREC+3,1:11),@("LIM=1E-"_LIM)
 For K=3:2 Do  Quit:($Translate(1/L,"-")<LIM)
 . ;
 . ;;; Set L=L*X*X,VALUE=VALUE+(1/(K*L)*SIGN),
 . ; Eli Reidler (28 June 1996)
 . Set L=L*X*X,VALUE=VALUE+(1/(K*L)*SIGN)
 . ;;;
 . ;
 . Set SIGN=SIGN*-1
 . Quit
 Set VALUE=$Select(FOLD:$%PI^MATH()/2-VALUE,1:VALUE)
 Set VALUE=$Select(SIGT<1:-VALUE,1:VALUE)
 Quit VALUE
 ;===
 ;
 ;
 ;


