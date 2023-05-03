16-Feb-1996, 16:54:35
Routine Save for Library function %CRC32^STRING
 ;
 ; The code below was approved in document X11/1998-32
 ;
CRC32(string,seed) ;
 ; Polynomial X**32 + X**26 + X**23 + X**22 +
 ;          + X**16 + X**12 + X**11 + X**10 +
 ;          + X**8  + X**7  + X**5  + X**4 +
 ;          + X**2  + X     + 1
 NEW I,J,R
 IF '$DATA(seed) SET R=4294967295
 ELSE  IF seed'<0,seed'>4294967295 SET R=4294967295-seed
 ELSE  SET $ECODE=",M28,"
 FOR I=1:1:$LENGTH(string) DO
 . SET R=$$XOR($ASCII(string,I),R,8)
 . FOR J=0:1:7 DO
 . . IF R#2 SET R=$$XOR(R\2,3988292384,32)
 . . ELSE  SET R=R\2
 . . QUIT
 . QUIT
 QUIT 4294967295-R
XOR(a,b,w) NEW I,M,R
 SET R=b,M=1
 FOR I=1:1:w DO
 . SET:a\M#2 R=R+$SELECT(R\M#2:-M,1:M)
 . SET M=M+M
 . QUIT
 QUIT R
 ; ===
 ;
 ;



