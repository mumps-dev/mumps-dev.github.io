2-Jul-96, 6:54:35
Routine Save for Trigonometry Library function $%SIN^MATH
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
SIN(X,PREC) ;
 New L,LIM,K,SIGN,VALUE
 ;
 ;;; Set:X[":" X=$%DMSDEC^MATH(X,12)
 ; Winfried Gerum (8 June 1995)
 ;    Comment: The official description does not mention than
 ;             the function may also be called with the first
 ;             parameter in degrees, minutes and seconds.
 Set:X[":" X=$%DMSDEC^MATH(X)
 ;;;
 ;
 Set PREC=$Get(PREC,11)
 Set X=X#(2*$%PI^MATH())
 Set (VALUE,L)=X,SIGN=-1
 Set LIM=$Select((PREC+3)'>11:PREC+3,1:11),@("LIM=1E-"_LIM)
 For K=3:2 Do  Quit:($Translate(L,"-")<LIM)  Set SIGN=SIGN*-1
 . Set L=L/(K-1)*X/K*X,VALUE=VALUE+(SIGN*L)
 . Quit
 Quit VALUE
 ;===
 ;
 ;
SIN(X) ;
 ; Winfried Gerum (8 June 1995)
 ;  Comment: This version of the function is
 ;           optimized for speed, not for precision.
 ;           The 'precision' parameter is not supported,
 ;           and the precision is at best 1 in 10**-9.
 ;           Note that this function does not accept its
 ;           parameter in degrees, minutes and seconds.
 ;
 New A,N,PI,R,SIGN,XX
 ;
 ; This approximation only works for 0 <= x <= pi/2
 ; so reduce angle to correct quadrant.
 ;
 Set PI=$%PI^MATH(),X=X#(PI*2),SIGN=1
 Set:X>PI X=2*PI-X,SIGN=-1
 ;
 ;;; Set:X*2<PI X=PI-X Set X=-PI/2+2
 ; Winfried Gerum (8 June 1995)
 Set:X*2<PI X=PI-X
 ;;;
 ;
 ;
 Set XX=X*X,A(1)=-0.4999999963,A(2)=0.0416666418
 Set A(3)=-0.0013888397,A(4)=0.0000247609,A(5)=-0.0000002605
 Set (X,R)=1 For N=1:1:5 Set X=X*XX,R=A(N)*X+R
 Quit R*SIGN
 ;===
 ;
 ;
 ;


