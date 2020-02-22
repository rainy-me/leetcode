class Solution:
    def isLongPressedName(self, name: str, typed: str) -> bool:
        nameIndex = typedIndex = 0
        nameLen = len(name)
        typedLen = len(typed)
        while 1:
            if nameIndex == nameLen:
                return True
            if typedIndex == typedLen:
                return False
            if name[nameIndex] == typed[typedIndex]:
                nameIndex += 1
                typedIndex += 1
            else:
                if typed[typedIndex] == typed[typedIndex-1]:
                    typedIndex += 1
                else:
                    return False
