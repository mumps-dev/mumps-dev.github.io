16-Feb-1996, 16:54:35
Routine Save for Library function %CRCCCITT^STRING
 ;
 ; The code below was approved in document X11/1998-32
 ;
CRCCCITT(string,seed) ;
 ; Polynomial x**16 + x**12 + x**5 + x**0
 NEW I,J,R
 IF '$DATA(seed) SET R=65535
 ELSE  IF seed'<0,seed'>65535 SET R=seed\1
 ELSE  SET $ECODE=",M28,"
 FOR I=1:1:$LENGTH(string) DO
 . SET R=$$XOR($ASCII(string,I)*256,R,16)
 . FOR J=0:1:7 DO
 . . SET R=R+R
 . . QUIT:R<65536
 . . SET R=$$XOR(4129,R-65536,13)
 . . QUIT
 . QUIT
 QUIT R
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



