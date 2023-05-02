2-Jul-96, 6:54:35
Routine Save for Matrix Mathematics Library function $%MTXUNIT^MATH
 ;
 ; Unless otherwise noted, the code below
 ; was approved in document X11/96-26
 ;
 ; If corrections have been applied,
 ; first the original line appears,
 ; with three semicolons at the beginning of the line.
 ;
 ; Then the source of the correction is acknowledged,
 ; then the corrected line appears, followed by a
 ; line containing three semicolons.
 ;
MTXUNIT(R,N,SPARSE) ;
 ; Create a unit matrix R[N,N]
 IF $GET(N)<1 QUIT 0
 ;
 NEW ROW,COL
 FOR ROW=1:1:N FOR COL=1:1:N DO
 . KVALUE R(ROW,COL)
 . IF $GET(SPARSE) QUIT:ROW'=COL
 . SET R(ROW,COL)=$SELECT(ROW=COL:1,1:0)
 . QUIT
 QUIT 1
 ;===
 ;
 ;
 ;


