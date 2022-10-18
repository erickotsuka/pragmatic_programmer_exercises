import re

digit = "[0-9]"
m_tens = "[0-5]"
h_tens = "[0-2]"
ampm = "(a|p)m"
hour = f"({h_tens}{digit})|{digit}"
minute = f"{m_tens}{digit}"
time = f"^(({hour}):({minute})({ampm})?)|({hour}{ampm})"

while True:
    if re.fullmatch(time, input()) == None:
        print("Invalid")
    else:
        print("Ok")
