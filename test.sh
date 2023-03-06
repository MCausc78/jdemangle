#!/bin/bash
LINES="$(cat <<EOF
Z
B
C
S
I
J
F
D
Ljava/lang/String;
[Z
[B
[C
[S
[I
[J
[F
[D
[Ljava/lang/String;
[[Z
[[B
[[C
[[S
[[I
[[J
[[F
[[D
[[Ljava/lang/BigInteger;
()V
(Ljava/lang/String;)V
(II)I
_im_invalid
EOF
)"

EXPECTING="$(cat <<EOF
boolean
byte
char
short
int
long
float
double
java.lang.String
boolean[]
byte[]
char[]
short[]
int[]
long[]
float[]
double[]
java.lang.String[]
boolean[][]
byte[][]
char[][]
short[][]
int[][]
long[][]
float[][]
double[][]
java.lang.BigInteger[][]
void()
void(java.lang.String)
int(int, int)
_im_invalid
EOF
)"

printf "lines: \"%s\"\n" "$LINES"
OUTPUT="$(printf "%s" "$LINES" | ./target/release/jdemangle)"
if [[ "$OUTPUT" != "$EXPECTING" ]]
then
	printf "\nfail: \"%s\"\n\nexpected: \"%s\"\n" "$OUTPUT" "$EXPECTING"
else
	printf "\nsuccess: \"%s\"\n" "$OUTPUT"
fi

# echo -ne "Z\nB\nC\nS\nI\nJ\nF\nD\nLjava/lang/String;\n[Z\n[B\n[C\n[S\n[I\n[J\n[F\n[D\n[Ljava/lang/String;\n[[Z\n[[B\n[[C\n[[S\n[[I\n[[J\n[[F\n[[D\n[[Ljava/lang/BigInteger;\n()V\n(Ljava/lang/String;)V\n(II)I" | ./target/release/jdemangle
