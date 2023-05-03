2-Jul-96, 6:54:35
Routine Save for Hyperbolic Trigonometry Library function $%COSH^MATH
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
COSH(X,PREC) ;
 ;
 ;;; New F,I,P,R,T,XX
 ; Winfried Gerum (8 June 1995)
 New E,F,I,P,R,T,XX
 ;;;
 ;
 Set PREC=$Get(PREC,11)+1
 Set @("E=1E-"_PREC)
 Set XX=X*X,F=1,(P,R,T)=1,I=1
 For  Set T=T*XX,F=I+1*I*F,R=T/F+R,P=P-R/R,I=I+2 If -E<P,P<E Quit
 Quit R
 ;===
 ;
 ;
 ;


