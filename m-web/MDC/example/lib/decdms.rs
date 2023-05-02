2-Jul-96, 6:54:35
Routine Save for General Mathematics Library function $%DECDMS^MATH
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
DECDMS(X,PREC) ;
 Set PREC=$Get(PREC,5)
 Set X=X#360*3600
 Set X=+$Justify(X,0,$Select((PREC-$Length(X\1))'<0:PREC-$Length(X\1),1:0))
 Quit X\3600_":"_(X\60#60)_":"_(X#60)
 ;===
 ;
 ;
 ;


