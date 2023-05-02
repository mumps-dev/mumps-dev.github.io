2-Jul-96, 6:54:35
Routine Save for Matrix Mathematics Library function $%MTXADD^MATH
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
MTXADD(A,B,R,ROWS,COLS) ;
 ; Add A[ROWS,COLS] to B[ROWS,COLS],
 ; result goes to R[ROWS,COLS]
 IF $DATA(A)<10 QUIT 0
 IF $DATA(B)<10 QUIT 0
 IF $GET(ROWS)<1 QUIT 0
 IF $GET(COLS)<1 QUIT 0
 ;
 NEW ROW,COL,ANY
 FOR ROW=1:1:ROWS FOR COL=1:1:COLS DO
 . KVALUE R(ROW,COL) SET ANY=0
 . SET:$DATA(A(ROW,COL))#2 ANY=1
 . SET:$DATA(B(ROW,COL))#2 ANY=1
 . SET:ANY R(ROW,COL)=$GET(A(ROW,COL))+$GET(B(ROW,COL))
 . QUIT
 QUIT 1
 ;===
 ;
 ;
 ;


