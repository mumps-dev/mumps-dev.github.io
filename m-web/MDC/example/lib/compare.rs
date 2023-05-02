16-Feb-1996, 16:54:35
Routine Save for Library function %COMPARE^STRING
 ;
 ; The code below was approved in document X11/1996-74
 ;
COMPARE(A,B,CHARMOD) NEW X,Y
 ;
 ; Assume current collation,
 ; i.e. ]] if no CHARMOD specified
 ;
 IF $GET(CHARMOD)="" QUIT $SELECT(A=B:0,A]]B:1,1:-1)
 ;
 ; Otherwise need to override it and do
 ; string compare on collation value
 ;
 SET X=$%COLLATE^STRING(A,CHARMOD)
 SET Y=$%COLLATE^STRING(B,CHARMOD)
 QUIT $S(X=Y:0,X]Y:1,1:-1)
 ; ===
 ;
 ;



