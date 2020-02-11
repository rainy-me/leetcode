# @see https://leetcode.com/problems/transpose-file/discuss/55502/AC-solution-using-awk-and-statement-just-like-C.
# @see https://www.grymoire.com/Unix/Awk.html

awk '
{
    for (i = 1; i <= NF; i++) {
        if(NR == 1) {
            s[i] = $i;
        } else {
            s[i] = s[i] " " $i;
        }
    }
}
END {
    for (i = 1; s[i] != ""; i++) {
        print s[i];
    }
}' file.txt