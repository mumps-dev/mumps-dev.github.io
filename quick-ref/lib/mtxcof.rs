2-Jul-96, 6:54:35
Routine Save for Matrix Mathematics Library function $%MTXCOF^MATH
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
MTXCOF(A,I,K,N) ;
 ; Compute cofactor for element [i,k]
 ; in matrix A[N,N]
 NEW T,R,C,RR,CC
 SET CC=0 FOR C=1:1:N DO:C'=K
 . SET CC=CC+1,RR=0
 . FOR R=1:1:N SET:R'=I RR=RR+1,T(RR,CC)=$GET(A(R,C))
 . QUIT
 QUIT $%MTXDET^MATH(.T,N-1)
 ;===
 ;
 ;
 ;


