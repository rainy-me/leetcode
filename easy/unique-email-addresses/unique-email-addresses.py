class Solution:
    def numUniqueEmails(self, emails):
        names = []
        for email in emails:
            name, domain = email.split('@')
            name = name.split('+', 1)[0].replace('.', '') + '🤣' + domain
            names.append(name)
        return len(set(names))


if __name__ == '__main__':
    print(Solution().numUniqueEmails(
        ["test.email+alex@leetcode.com", "test.email.leet+alex@code.com"]))
