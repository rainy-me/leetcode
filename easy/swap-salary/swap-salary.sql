-- ASCII('f') ^ ASCII('m') === 11
update salary set sex = CHAR(11 ^ ASCII(sex));