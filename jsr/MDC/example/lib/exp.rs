2-Jul-96, 6:54:35
Routine Save for General Mathematics Library function $%EXP^MATH
 ;
 ; Unless otherwise noted, the code below
 ; was approved in document X11/95-11
 ;
 ; If corrections have been applied,
 ; first the original line appears,
 ; with three semicolons at the beginning of the line.
 ;
 ; Then the source of the correction is acknowledged,
 ; then the corrected line appears, followed by a
 ; line containing three semicolons.
 ;
EXP(X,PREC) ;
 New L,LIM,K,VALUE
 Set PREC=$Get(PREC,11)
 Set L=X,VALUE=X+1
 Set LIM=$Select((PREC+3)'>11:PREC+3,1:11),@("LIM=1E-"_LIM)
 For K=2:1 Set L=L*X/K,VALUE=VALUE+L Quit:($Translate(L,"-")<LIM)
 Quit VALUE
 ;===
 ;
 ;
 ;


