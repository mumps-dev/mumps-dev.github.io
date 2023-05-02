5-Apr-2000  6:54:35
Routine Save for String Library function $%FORMAT^STRING
 ;
 ; The code below was approved in document X11/SC13/TG2/1999-1
 ;
 ; EdM made some modifications in the context of entering
 ; the MDC Type A Extension into the draft standard.
 ;
FORMAT(V,S) ;
 ;
 ; EdM 5 Apr 2000: added 'sign' to the list
 New lo,mask,out,p,pos,sign,spec,up,v1,v2,val,x
 ;
 Set lo="abcdefghijklmnopqrstuvwxyz"
 Set up="ABCDEFGHIJKLMNOPQRSTUVWXYZ"
 ;
 ; Array spec() contains the formatting directives
 ;
 ; First set defaults
 ;
 Set spec("CS")="$" ; Currency symbol
 Set spec("DC")="." ; Decimal separator
 Set spec("EC")="*" ; Error character
 Set spec("SL")="," ; Separator characters > 1
 Set spec("FS")=" " ; Fill string
 ;
 ; Other specifiers may be
 ;  FM = Format Mask
 ;  FO = Fill On/Off
 ;  SR = Separator characters < 1
 ;
 ; Then Inherit properties from System,
 ; overwriting the defaults
 ;
 Set x="" For  Set x=$Order(^$System($System,"FORMAT",x)) Quit:x=""  Do
 . Set spec(x)=^$System($System,"FORMAT",x)
 . Quit
 ;
 ; Then Inherit properties from current process
 ; overwriting the system and the defaults
 ;
 Set x="" For  Set x=$Order(^$Job($Job,"FORMAT",x)) Quit:x=""  Do
 . Set spec(x)=^$Job($Job,"FORMAT",x)
 . Quit
 ;
 ; Then look at actual parameters
 ; overwriting anything else
 ;
 Set S=$Get(S) For  Quit:S=""  Do
 . New e,i,str,v
 . Set x=$Piece(S,"=",1)
 . Set i=$Length(x)+2,str=0,v=""
 . Set:x="" i=1
 . For i=i:1:$Length(S)+1 Do  Quit:'i
 . . Set e=$Extract(S_":",i)
 . . If 'str,e=":" Set S=$Extract(S,i+1,$Length(S)),i=0 Quit
 . . Set v=v_e Quit:e'=""""
 . . Set str=1-str
 . . Quit
 . If i>$Length(S) Set S=""
 . If x'="",v'="" Set @("spec($Translate(x,lo,up))="_v) Quit
 . Set $ECode=",M28,"
 . Quit
 ;
 ; Make certain that DC and EC are non-empty
 ; and not longer than 1 character
 ;
 Set spec("DC")=$Extract(spec("DC")_".",1)
 Set spec("EC")=$Extract(spec("EC")_"*",1)
 ;
 Set val=$Get(V),(mask,out)=$Get(spec("FM"))
 If mask="" Quit val
 ;
 ; Currency string
 ;
 Set x=spec("CS")
 Set pos=0 For  Set pos=$Find(mask,"c",pos) Quit:pos<1  Do
 . Set $Extract(out,pos-1)=$Extract(x,1)
 . Set x=$Extract(x,2,$Length(x))_$Extract(x,1)
 . Quit
 ;
 ; Sign
 ;
 w !,mask ; ~~~
 ; EdM 5 Apr 2000: added assignments to 'sign'
 Set x=$Select(val>0:"+",val<0:"-",1:" "),sign=0
 Set pos=0 For  Set pos=$Find(mask,"+",pos) Quit:pos<1  Do
 . Set $Extract(out,pos-1)=x,sign=1
 . Quit
 Set pos=0 For  Set pos=$Find(mask,"-",pos) Quit:pos<1  Do
 . Set $Extract(out,pos-1)=$Select(x="-":x,1:" "),sign=1
 . Quit
 Set:(mask["(")!(mask[")") sign=1
 If x'="-" Set out=$Translate(out,"()","  ")
 ;
 ; Decimal separator
 ;
 Set pos=$Find(mask,"d")
 Do:pos'<1
 . Set $Extract(out,pos-1)=spec("DC")
 . For  Set pos=$Find(mask,"d",pos) Quit:pos<1  Do
 . . Set $Extract(out,pos-1)=spec("EC")
 . . Quit
 . Quit
 ;
 ; Right (default, format letter "n") or
 ; left (format letter "l") adjustment?
 ;
 If mask["l",mask["n" Set $ECode=",M28,"
 ;
 ; Left and Right Separators
 ;
 Set v1=$Piece(val,".",1),v2=$Piece(val,".",2)
 ; EdM 5 Apr 2000: added 'sign' as post-condition
 Set:sign v1=$Translate(v1,"-")
 If mask'["l" Do
 . Set x="" For p=1:1:$Length(v1) Set x=$Extract(v1,p)_x
 . Set v1=x
 . Quit
 ;
 Set pos=$Find(mask,"d") Set:pos<1 pos=$Length(mask)+2
 ;
 ; Integer part and Left separators
 ;
 Set x=spec("SL")
 Set p(1)=pos-2,p(2)=-1,p(3)=1
 Set:mask["l" p(1)=1,p(2)=1,p(3)=pos-2
 For p=p(1):p(2):p(3) Do
 . If "fln"[$Extract(mask,p) Do
 . . Set $Extract(out,p)=$Extract(v1,1)
 . . Set v1=$Extract(v1,2,$Length(v1))_spec("FS")
 . . If $Translate(v1,spec("FS"))="" Set x=spec("FS")
 . . Quit
 . If $Extract(mask,p)="s" Do
 . . ; EdM 5 Apr 2000: added exception
 . . If $Extract(v1,1)="-" Do  Quit
 . . . Set $Extract(out,p)=$Extract(v1,1)
 . . . Set v1=$Extract(v1,2,$Length(v1))_spec("FS")
 . . . Quit
 . . Set $Extract(out,p)=$Extract(x,1)
 . . Set x=$Extract(x,2,$Length(x))_$Extract(x,1)
 . Quit
 ;
 ; Fractional part and Right separators
 ;
 Set x=$Get(spec("SR"),spec("SL"))
 Set:v2="" v2=0
 For p=pos:1:$Length(mask) Do
 . If "fn"[$Extract(mask,p) Do
 . . Set $Extract(out,p)=$Extract(v2,1)
 . . Set v2=$Extract(v2,2,$Length(v2))_"0"
 . . Quit
 . If $Extract(mask,p)="s" Do
 . . Set $Extract(out,p)=$Extract(x,1)
 . . Set x=$Extract(x,2,$Length(x))_$Extract(x,1)
 . . Quit
 . Quit
 ;
 ; Fill String
 ;
 Set x=$Get(spec("FS"))
 For p=1:1:$l(mask) Do
 . Quit:"nf"'[$Extract(mask,p)
 . Quit:$Extract(out,p)'=" "
 . Set $Extract(out,p)=$Extract(x,1)
 . Set x=$Extract(x,2,$Length(x))_$Extract(x,1)
 . Quit
 ;
 ; Justification
 ;
 For x="+ | +","- | -","( | ("," )|) " Do
 . New find,repl
 . Set find=$Piece(x,"|",1),repl=$Piece(x,"|",2)
 . For  Quit:out'[find  Do
 . . Set out=$Piece(out,find,1)_repl_$Piece(out,find,2,$l(out)+2)
 . . Quit
 . Quit
 ;
 Quit out
