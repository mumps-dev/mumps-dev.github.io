2-Jul-96, 6:54:35
Routine Save for Complex Mathematics Library function $%CMUL^MATH
 ;
 ; Unless otherwise noted, the code below
 ; was approved in document X11/95-14
 ;
 ; If corrections have been applied,
 ; first the original line appears,
 ; with three semicolons at the beginning of the line.
 ;
 ; Then the source of the correction is acknowledged,
 ; then the corrected line appears, followed by a
 ; line containing three semicolons.
 ;
CMUL(X,Y) ;
 New XIM,XRE,YIM,YRE
 Set XRE=+X,XIM=+$Piece(X,"%",2)
 Set YRE=+Y,YIM=+$Piece(Y,"%",2)
 Quit XRE*YRE-(XIM*YIM)_"%"_(XRE*YIM+(XIM*YRE))
 ;===
 ;
 ;
 ;


