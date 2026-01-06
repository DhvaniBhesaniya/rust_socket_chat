import time
import random
import string
import json
import requests
import os
import threading
 
FILE_NAME = "generated.json"
SAVE_INTERVAL = 120  # 5 minutes (in seconds)
 
# --- Load from file if exists ---
if os.path.exists(FILE_NAME):
    with open(FILE_NAME, "r") as f:
        try:
            generated_coupons = set(json.load(f))
            print(f"Loaded {len(generated_coupons)} from file.")
        except json.JSONDecodeError:
            generated_coupons = set()
else:
    generated_coupons = set()
 
 
def save_coupons():
    """Save generated data to file"""
    with open(FILE_NAME, "w") as f:
        json.dump(list(generated_coupons), f, indent=4)
    print(f"[SAVE] {len(generated_coupons)} stored in {FILE_NAME}")
 
 
def save_periodically():
    """Background auto-saver"""
    while True:
        time.sleep(SAVE_INTERVAL)
        save_coupons()
 
 
# Start background saving thread
threading.Thread(target=save_periodically, daemon=True).start()
 
def generate_coupons(count=25):
    numbers = string.digits
    letters = string.ascii_uppercase
    new_coupons = []
 
    while len(new_coupons) < count:
        # prefix = random.choice([
        #     "7L3"     ])
 
        # for i in range(1,8):
        #     if i % 2 == 0:
        #         coupon += random.choice(numbers)
        #     else:
        #         coupon += random.choice(letters)
         # Observed "template" from analysis
        template = [
      random.choice("T"),     # pos1 (rare digits instead of 0/9)
    random.choice("89760521"),     # pos2 (rare letters)
  random.choice("01654897"),           # pos3 (0,1,6,8,9)
        random.choice("4506789"),           # pos4
        random.choice("3450789"),           # pos5
        random.choice("34650789"),          # pos6
        random.choice("0124689"),         # pos7
        random.choice("12467089"),         # pos8
        random.choice("0146789"),         # pos9
        random.choice("23460789"),         # pos10
        random.choice("935086"),      
     # pos10
]
        coupon = "".join(template)
 
        if coupon not in generated_coupons:
            generated_coupons.add(coupon)
            new_coupons.append(coupon)
 
    return new_coupons
 
 
# def generate_coupon():
#     numbers = string.digits
#     letters = string.ascii_uppercase
#     # Observed "template" from analysis
#     template = [
#         '9',                  # pos1: 9 is most frequent
#         random.choice('BY'),  # pos2: B or Y often appear
#         '0',                  # pos3: 0 dominates
#         'N',                  # pos4: N frequent
#         '9',                  # pos5: 9 common
#         random.choice('57'),  # pos6: 5 or 7 strong
#         '7',                  # pos7: 7 frequent
#         random.choice('XN'),  # pos8: X or N repeat
#         '9',                  # pos9: 9 dominates
#         random.choice('BM')   # pos10: B or M frequent
#     ]
 
#     return "".join(template)
 
def check_coupon(coupons,user):
    coupons = "%2C".join(coupons)
    url = f"https://www.jiomart.com/mst/rest/v1/5/cart/check_coupons?cart_id={user['cartid']}&coupon_code_csv={coupons}"
    headers = {
        "authtoken": user['authtoken'],
        "userid": str(user['userid']),
        "pin": '380015',
        "Accept": "application/json, text/plain, */*"
    }
 
    response = requests.get(url, headers=headers)
    # print(response.text)
     # Debug: check what actually came back
    print("Status Code:", response.status_code)
    # print("Response Headers:", response.headers)
    try:
        return response.json()
    except ValueError:
        print(response.text)
        print("Response is not valid JSON. Raw text:")
        # print(response.text)
        return None
 
def main():
    i = 0;
    # //1st i am pote
    # 2nd i am pote ni mummy nu
    #3rd om kumar chikhaliya nu
    #4th bhdaresh bhai chuhan nu
    #5th deep bhai padariya sarkar nu
    #6th kashyap bhai ajudiya urfe vahivatiya nu
    users =[
{"userid":177195981,"cartid":672334655,"authtoken":'6495dc1149b19bc42004a87d592a25fc006f2574507634592'},
]
    while True:
        for user in users:
            coupons = generate_coupons()
            # print(coupons)
            response = check_coupon(coupons,user)
            if response.get("status") == "success" and isinstance(response.get("result"), list):
                for obj in response["result"]:
                    if obj.get("applicable") is True:  # only append if we have something after filtering
                        print("==================================================>",obj)
 
                        # --- classify by min_order_value ---
                        min_order_value = obj.get("mov_info", {}).get("min_order_value")
                        if min_order_value == 51:
                            filename = "test_min_51.json"
                        elif min_order_value == 199:
                            filename = "test_min_199.json"
                        else:
                            filename = "test_min_other.json"
 
                        with open(filename, "r") as file:
                            data = json.load(file)
                        data["result"].append(obj)
 
                        with open(filename, "w") as file:
                            json.dump(data, file, indent=4)
 
                    elif obj.get("reason_code") == "INVALID_COUPON":
                        with open("test_invalid.json", "r") as file:
                            data = json.load(file)
                        data["result"].append(obj)
                        with open("test_invalid.json", "w") as file:
                            json.dump(data, file, indent=4)
            else:
                print("Invalid", response.get("status"))
        i+=1
        print(f"{i}th round")
        # print("Sleeping 11s before next round...")
        time.sleep(11)
 
 
 
if __name__ == "__main__":
    main()
    # print(generate_coupons(20))
 

730151