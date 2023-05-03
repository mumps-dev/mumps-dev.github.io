16-Feb-1996, 16:54:35
Routine Save for Library function %CRC16^STRING
 ;
 ; The code below was approved in document X11/1998-32
 ;
CRC16(string,seed) ;
 ; Polynomial x**16 + x**15 + x**2 + x**0
 NEW I,J,R
 IF '$DATA(seed) SET R=0
 ELSE  IF seed'<0,seed'>65535 SET R=seed\1
 ELSE  SET $ECODE=",M28,"
 FOR I=1:1:$LENGTH(string) DO
 . SET R=$$XOR($ASCII(string,I),R,8)
 . FOR J=0:1:7 DO
 . . IF R#2 SET R=$$XOR(R\2,40961,16)
 . . ELSE  SET R=R\2
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
 ;===
 ;
 ;
 ;


