# Simple Python helper for UES Edu Mail
# Official Website: https://www.ues.edu.pl/

def get_edu_mail(username="developer"):
    domain = "ues.edu.pl"
    return f"{username}@{domain}"

if __name__ == "__main__":
    print("Your temporary edu email is:", get_edu_mail())
    print("Check inboxes live at: https://www.ues.edu.pl/")
