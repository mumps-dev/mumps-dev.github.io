2-Jul-96, 6:54:35
Routine Save for Hyperbolic Trigonometry Library function $%ARCCOTH^MATH
 ;
 ; Unless otherwise noted, the code below
 ; was approved in document X11/95-13
 ;
 ; If corrections have been applied,
 ; first the original line appears,
 ; with three semicolons at the beginning of the line.
 ;
 ; Then the source of the correction is acknowledged,
 ; then the corrected line appears, followed by a
 ; line containing three semicolons.
 ;
ARCCOTH(X,PREC) ;
 New L1,L2
 ;
 ;;;
 ; Winfried Gerum (8 June 1995)
 ; Alan Frank (October 1995)
 Set PREC=$Get(PREC,11)
 ;;;
 ;
 Set L1=$%LOG^MATH(X+1,PREC)
 Set L2=$%LOG^MATH(X-1,PREC)
 Quit L1-L2/2
 ;===
 ;
 ;
 ;


