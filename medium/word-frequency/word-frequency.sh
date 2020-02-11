# @see https://leetcode.com/problems/word-frequency/discuss/55443/My-simple-solution-(one-line-with-pipe)
cat words.txt | xargs -n1 | sort | uniq -c | sort -rn | awk '{ print $2, $1 }'

# @see https://leetcode.com/submissions/detail/302377621/
sed -ze 's/\s\s*/\n/g' words.txt | sort | uniq -c | sort -r -n | sed -e 's/^ *\([0-9]\+\) *\(.*\)$/\2 \1/'