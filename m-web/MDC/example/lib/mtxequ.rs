2-Jul-96, 6:54:35
Routine Save for Matrix Mathematics Library function $%MTXEQU^MATH
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
MTXEQU(A,B,R,N,M) ;
 ; Solve matrix equation A [M,M] * R [M,N] = B [M,N]
 IF $GET(M)<1 QUIT ""
 IF $GET(N)<1 QUIT ""
 IF '$%MTXDET^MATH(.A,M) QUIT 0
 ;
 NEW I,I1,J,J1,J2,K,L,T,T1,T2,TEMP,X
 ;
 SET X=$%MTXCOPY^MATH(.A,.T,N,N)
 SET X=$%MTXCOPY^MATH(.B,.R,N,M)
 ;
 ; Reduction of matrix A
 ; Steps of reduction are counted by index K
 ;
 FOR K=1:1:N-1 DO
 . ;
 . ; Search for largest coefficient of T
 . ; (denoted by TEMP)
 . ; in first column of reduced system
 . ;
 . SET TEMP=0,J2=K
 . FOR J1=K:1:N DO
 . . QUIT:$TRANSLATE($GET(T(J1,K)),"-")>$TRANSLATE(TEMP,"-")
 . . SET TEMP=T(J1,K),J2=J1
 . . QUIT
 . ;
 . ; Exchange row number K with row number J2,
 . ; if necessary
 . ;
 . DO:J2'=K
 . . ;
 . . FOR J=K:1:N DO
 . . . SET T1=$GET(T(K,J)),T2=$GET(T(J2,J))
 . . . KILL T(K,J),T(J2,J)
 . . . IF T1'="" SET T(J2,J)=T1
 . . . IF T2'="" SET T(K,J)=T2
 . . . QUIT
 . . FOR J=1:1:M DO
 . . . SET T1=$GET(R(K,J)),T2=$GET(R(J2,J))
 . . . KILL R(K,J),R(J2,J)
 . . . IF T1'="" SET R(J2,J)=T1
 . . . IF T2'="" SET R(K,J)=T2
 . . . QUIT
 . . QUIT
 . ;
 . ; Actual reduction
 . ;
 . FOR I=K+1:1:N DO
 . . FOR J=K+1:1:N DO
 . . . QUIT:'$GET(T(K,K))
 . . . SET T(I,J)=-$GET(T(K,J))*$GET(T(I,K))/T(K,K)+$GET(T(I,J))
 . . . QUIT
 . . FOR J=1:1:M DO
 . . . QUIT:'$GET(T(K,K))
 . . . SET R(I,J)=-$GET(R(K,J))*$GET(T(I,K))/T(K,K)+$GET(R(I,J))
 . . . QUIT
 . . QUIT
 . QUIT
 ;
 ; Backsubstitution
 ;
 FOR J=1:1:M DO
 . IF $GET(T(N,N)) SET R(N,J)=$GET(R(N,J))/T(N,N)
 . IF N-1>0 FOR I1=1:1:N-1 DO
 . . SET I=N-I1
 . . FOR L=I+1:1:N DO
 . . . SET R(I,J)=-$GET(T(I,L))*$GET(R(L,J))+$GET(R(I,J))
 . . . QUIT
 . . IF $GET(T(I,I)) SET R(I,J)=$GET(R(I,J))/$GET(T(I,I))
 . . QUIT
 . QUIT
 QUIT $SELECT(M=N:$%MTXDET^MATH(.R,M),1:1)
 ;===
 ;
 ;
 ;


