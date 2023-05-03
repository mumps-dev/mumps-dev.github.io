2-Jul-96, 6:54:35
Routine Save for Hyperbolic Trigonometry Library function $%ARCCOSH^MATH
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
ARCCOSH(X,PREC) ;
 If X<1 Set $Ecode=",M28,"
 New SQ
 ;
 ;;;
 ; Winfried Gerum (8 June 1995)
 ; Alan Frank (October 1995)
 Set PREC=$Get(PREC,11)
 ;;;
 ;
 Set SQ=$%SQRT^MATH(X*X-1,PREC)
 Quit $%LOG^MATH(X+SQ,PREC)
 ;===
 ;
 ;
 ;


