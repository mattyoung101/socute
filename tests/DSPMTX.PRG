;
;	DSP library program Type 2
;
;					'94/09/20      F.Sonobe
;
;           RAM0              RAM1              RAM2              RAM3
;       +----------+      +----------+      +----------+      +----------+
;     00| IN Matrix|    00|          |    00|          |    00| X or cos |
;     01|OUT Matrix|      |  Matrix  |      | Current  |      | Y or son |
;       |----------|      |   Work   |      |  Matrix  |      | Z or-sin |
;     02| Operat.1 |      |          |      |          |      |----------|
;       | X or cos |      |          |      |          |    03|  $10000  |
;       | Y or son |      |----------|      |----------|      |----------|
;       | Z or-sin |    0C|          |    0C|          |    04|          |
;       |----------|      |          |      |          |      |          |
;     06| Operat.2 |      |          |      |          |      |          |
;       | X or cos |      |          |      |          |      |          |
;       | Y or son |      |          |      |          |      |          |
;       | Z or-sin |      |          |      |          |      |          |
;       |----------|      |          |      |          |      |          |
;     0A|     .    |      |          |      |          |      |          |
;       |     .    |      |          |      |          |      |          |
;       |     .    |      |          |      |          |      |          |
;       |     .    |      |          |      |          |      |          |
;       |----------|      |          |      |          |      |          |
;     3A| Operat.15|      |          |      |          |      |          |
;       | X or cos |      |          |      |          |      |          |
;       | Y or son |      |          |      |          |      |          |
;       | Z or-sin |      |          |      |          |      |          |
;       |----------|      |          |      |          |      |          |
;     3E|          |      |          |      |          |      |          |
;       |          |      |          |      |          |      |          |
;       +----------+      +----------+      +----------+      +----------+
;
;
ONE	=	$10000		; =1
MSZ	=	12		; Matrix Size
;
;	RAM0
IMP	=	$00		; Input Matrix Pointer
OMP	=	$01		; Output Matrix pointer
OPR	=	$02		; Operation
;
;	RAM1
MWK	=	$00		; Matrix Work
;
;	RAM2
CM	=	$00		; Current Matrix
;
;	RAM3
PTB	=	$00		; Paramater Table
ONEP	=	$03		; =1
;
;>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
;
	ORG	0
;
START
;
	                                                 MOV IMP,CT0
	                                                 MOV MC0,RA0
	                                                 MOV CM,CT2
	DMA D0,M2,MSZ
WTA1	JMP T0,WTA1
	                                                 MOV ONEP,CT3
	MVI ONE,MC3
PLP
	                                      MOV MC0,A  MOV PTB,CT3
	                                                 MOV MC0,MC3
	                                                 MOV MC0,MC3
	                                                 MOV MC0,MC3
	                                      MOV M0,A   MOV #1,PL
	SUB                                   MOV M0,A   MOV #2,PL
	JMP Z,F_CLR
	SUB                                   MOV M0,A   MOV #3,PL
	JMP Z,F_RZM
	SUB                                   MOV M0,A   MOV #4,PL
	JMP Z,F_MM
	SUB                                   MOV M0,A   MOV #5,PL
	JMP Z,F_RMX
	SUB                                   MOV M0,A   MOV #6,PL
	JMP Z,F_RMY
	SUB
	JMP Z,F_RMZ
	                                                 MOV CM,CT2
	                                                 MOV OMP,CT0
	                                                 MOV MC0,WA0
	DMA M2,D0,MSZ
WTA2	JMP T0,WTA2
	NOP
	ENDI

;
;////////////////////////////////////////////////////////////////////////////
;
;    ClearMatrix(Void)
;
;    RAM2 : matrix
;
F_CLR  			; Clear Matrix
	                                                 MOV CM,CT2
	                                                 MOV MSZ-1,LOP
	LPS
	                                                 MOV #0,MC2
;
	                                                 MOV CM,CT2
	MVI ONE,MC2
	                                                 MOV CM+5,CT2
	MVI ONE,MC2
	                                                 MOV CM+10,CT2
	JMP PLP
	MVI ONE,MC2

;
;////////////////////////////////////////////////////////////////////////////
;
;    ReverseZ Matrix(Viod)
;
;    RAM2 : matrix
;
F_RZM			; Reverse Signe Z Matrix
	                                                 MOV CM+2,CT2
	                MOV M2,P              CLR A
	SUB                                              MOV ALL,MC2
	                                                 MOV CM+6,CT2
	                MOV M2,P              CLR A
	SUB                                              MOV ALL,MC2
	                                                 MOV CM+10,CT2
	                MOV M2,P              CLR A
	JMP PLP
	SUB                                              MOV ALL,MC2

;
;////////////////////////////////////////////////////////////////////////////
;
;    MoveMatrix(Fixed32 x, Fixed32 y, Fixed32 z)
;
;    RAM2 : matrix
;    RAM3 : x, y, z
;
F_MM			; Move Matrix
	                                                 MOV PTB,CT3
	                                                 MOV CM,CT2
	     MOV MC2,X             MOV MC3,Y       
	     MOV MC2,X  MOV MUL,P  MOV MC3,Y  CLR A
	AD2  MOV MC2,X  MOV MUL,P  MOV MC3,Y  MOV ALU,A
	AD2  MOV M2,X   MOV MUL,P  MOV M3,Y   MOV ALU,A  MOV PTB,CT3
	AD2             MOV MUL,P             MOV ALU,A
	AD2                                              MOV ALH,MC2
	     MOV MC2,X             MOV MC3,Y       
	     MOV MC2,X  MOV MUL,P  MOV MC3,Y  CLR A
	AD2  MOV MC2,X  MOV MUL,P  MOV MC3,Y  MOV ALU,A
	AD2  MOV M2,X   MOV MUL,P  MOV M3,Y   MOV ALU,A  MOV PTB,CT3
	AD2             MOV MUL,P             MOV ALU,A
	AD2                                              MOV ALH,MC2
	     MOV MC2,X             MOV MC3,Y       
	     MOV MC2,X  MOV MUL,P  MOV MC3,Y  CLR A
	AD2  MOV MC2,X  MOV MUL,P  MOV MC3,Y  MOV ALU,A
	AD2  MOV M2,X   MOV MUL,P  MOV M3,Y   MOV ALU,A  MOV PTB,CT3
	AD2             MOV MUL,P             MOV ALU,A
	JMP PLP
	AD2                                              MOV ALH,MC2

;
;////////////////////////////////////////////////////////////////////////////
;
;    RotateMatrixX(Fixed32 sinNum, Fixed32 cosNum)
;
;    RAM1 : work
;    RAM2 : matrix
;    RAM3 : cos, sin, (-sin)
;
;
F_RMX			; Rotate Matrix X
	                                                 MOV PTB+1,CT3
	                MOV MC3,P             CLR A
	SUB                                              MOV ALL,MC3
	                                                 MOV MWK,CT1
	                                                 MOV CM,CT2
	                                                 MOV MSZ-1,LOP
	LPS
	                                                 MOV MC2,MC1
	                                                 MOV PTB,CT3
	                                                 MOV MWK+1,CT1
	     MOV MC3,X             MOV MC1,Y             MOV CM+1,CT2
	     MOV MC3,X  MOV MUL,P  MOV M1,Y   CLR A
	AD2  MOV MC3,X  MOV MUL,P  MOV M2,Y   MOV ALU,A  MOV PTB,CT3
	AD2  MOV M3,X   MOV MUL,P  MOV M1,Y   CLR A      MOV ALH,MC2
	AD2             MOV MUL,P             MOV ALU,A  MOV MWK+5,CT1
	AD2  MOV MC3,X             MOV MC1,Y             MOV ALH,MC2
	     MOV MC3,X  MOV MUL,P  MOV M1,Y   CLR A      MOV CM+5,CT2
	AD2  MOV MC3,X  MOV MUL,P  MOV M2,Y   MOV ALU,A  MOV PTB,CT3
	AD2  MOV M3,X   MOV MUL,P  MOV M1,Y   CLR A      MOV ALH,MC2
	AD2             MOV MUL,P             MOV ALU,A  MOV MWK+9,CT1
	AD2  MOV MC3,X             MOV MC1,Y             MOV ALH,MC2
	     MOV MC3,X  MOV MUL,P  MOV M1,Y   CLR A      MOV CM+9,CT2
	AD2  MOV MC3,X  MOV MUL,P  MOV M2,Y   MOV ALU,A  MOV PTB,CT3
	AD2  MOV M3,X   MOV MUL,P  MOV M1,Y   CLR A      MOV ALH,MC2
	AD2             MOV MUL,P             MOV ALU,A
	JMP PLP
	AD2                                              MOV ALH,MC2

;
;////////////////////////////////////////////////////////////////////////////
;
;    RotateMatrixY(Fixed32 sinNum, Fixed32 cosNum)
;
;    RAM1 : work
;    RAM2 : matrix
;    RAM3 : cos, sin, (-sin)
;
F_RMY			; Rotate Matrix Y
	                                                 MOV PTB+1,CT3
	                MOV MC3,P             CLR A
	SUB                                              MOV ALL,MC3
	                                                 MOV MWK,CT1
	                                                 MOV CM,CT2
	                                                 MOV MSZ-1,LOP
	LPS
	                                                 MOV MC2,MC1
	                                                 MOV CM,CT2
	                                                 MOV PTB+2,CT3
	                                                 MOV MWK+2,CT1
	     MOV MC3,X             MOV M1,Y              MOV PTB,CT3
	     MOV M3,X   MOV MUL,P  MOV M2,Y   CLR A      
	AD2  MOV MC3,X  MOV MUL,P  MOV M1,Y   MOV ALU,A  MOV MWK,CT1
	AD2  MOV MC3,X  MOV MUL,P  MOV M1,Y   CLR A      MOV ALH,MC2
	AD2             MOV MUL,P  MOV MC2,Y  MOV ALU,A  MOV MWK+6,CT1
	AD2  MOV MC3,X             MOV M1,Y              MOV ALH,MC2
	                                      MOV MC2,A  MOV PTB,CT3
	     MOV M3,X   MOV MUL,P  MOV M2,Y   CLR A
	AD2  MOV MC3,X  MOV MUL,P  MOV M1,Y   MOV ALU,A  MOV MWK+4,CT1
	AD2  MOV MC3,X  MOV MUL,P  MOV M1,Y   CLR A      MOV ALH,MC2
	AD2             MOV MUL,P  MOV MC2,Y  MOV ALU,A  MOV MWK+10,CT1
	AD2  MOV MC3,X             MOV M1,Y              MOV ALH,MC2
	                                      MOV MC2,A  MOV PTB,CT3
	     MOV M3,X   MOV MUL,P  MOV M2,Y   CLR A
	AD2  MOV MC3,X  MOV MUL,P  MOV M1,Y   MOV ALU,A  MOV MWK+8,CT1
	AD2  MOV MC3,X  MOV MUL,P  MOV M1,Y   CLR A      MOV ALH,MC2
	AD2             MOV MUL,P  MOV MC2,Y  MOV ALU,A
	JMP PLP
	AD2                                              MOV ALH,MC2

;
;////////////////////////////////////////////////////////////////////////////
;
;    RotateMatrixZ(Fixed32 sinNum, Fixed32 cosNum)
;
;    RAM1 : work
;    RAM2 : matrix
;    RAM3 : cos, sin, (-sin)
;
F_RMZ			; Rotate Matrix Z
	                                                 MOV PTB+1,CT3
	                MOV MC3,P             CLR A
	SUB                                              MOV ALL,MC3
	                                                 MOV MWK,CT1
	                                                 MOV CM,CT2
	                                                 MOV MSZ-1,LOP
	LPS
	                                                 MOV MC2,MC1
	                                                 MOV PTB,CT3
	                                                 MOV MWK,CT1
	     MOV MC3,X             MOV MC1,Y             MOV CM,CT2
	     MOV MC3,X  MOV MUL,P  MOV M1,Y   CLR A
	AD2  MOV MC3,X  MOV MUL,P  MOV M2,Y   MOV ALU,A  MOV PTB,CT3
	AD2  MOV M3,X   MOV MUL,P  MOV M1,Y   CLR A      MOV ALH,MC2
	AD2             MOV MUL,P             MOV ALU,A  MOV MWK+4,CT1
	AD2  MOV MC3,X             MOV MC1,Y             MOV ALH,MC2
	     MOV MC3,X  MOV MUL,P  MOV M1,Y   CLR A      MOV CM+4,CT2
	AD2  MOV MC3,X  MOV MUL,P  MOV M2,Y   MOV ALU,A  MOV PTB,CT3
	AD2  MOV M3,X   MOV MUL,P  MOV M1,Y   CLR A      MOV ALH,MC2
	AD2             MOV MUL,P             MOV ALU,A  MOV MWK+8,CT1
	AD2  MOV MC3,X             MOV MC1,Y             MOV ALH,MC2
	     MOV MC3,X  MOV MUL,P  MOV M1,Y   CLR A      MOV CM+8,CT2
	AD2  MOV MC3,X  MOV MUL,P  MOV M2,Y   MOV ALU,A  MOV PTB,CT3
	AD2  MOV M3,X   MOV MUL,P  MOV M1,Y   CLR A      MOV ALH,MC2
	AD2             MOV MUL,P             MOV ALU,A
	JMP PLP
	AD2                                              MOV ALH,MC2
;

 	ENDS
;
;<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<


