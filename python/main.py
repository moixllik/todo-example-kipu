import requests
import tkinter as tk
from tkinter import NSEW, ttk
from datetime import datetime

def db(event, uri, method=False, body=False):
    url = f"http://localhost:3320/{ event }/{ uri }"

    if method == "POST":
        re = requests.post(url, data=body)

    elif method == "DELETE":
        re = requests.delete(url)

    else:
        re = requests.get(url)

    d = re.text
    
    return d

class App(tk.Frame):
    def __init__(self, master):
        super().__init__(master)
        self.master.title("TODO")
        self.option_add("*font", "Sans 20")
        self.pack()

        self.columnconfigure(0, weight=1)
        self.columnconfigure(1, weight=1)

        self.e_desc = tk.Entry(self)
        self.e_desc.grid(column=0, row=0)

        b_add = ttk.Button(self, text="+", command=self.new)
        b_add.grid(column=1, row=0, sticky=NSEW)

        self.f_active = tk.LabelFrame(self)
        self.f_active.grid(column=0, columnspan=2, sticky=NSEW)

        self.f_completed = tk.LabelFrame(self)
        self.f_completed.grid(column=0, columnspan=2, sticky=NSEW)

        self.get_all()

    def get_all(self):
        d = db("list", "todo/0/desc-").splitlines()
        
        for uri in d:
            id = uri.replace("/data/todo/desc-", "")
            re_ok = requests.get("http://localhost:3320/data/todo/ok-" + id)
            re_desc = requests.get("http://localhost:3320/data/todo/desc-" + id)

            if re_ok.text == "":
                self.add(self.f_active, id, re_desc.text, False)
            else:
                self.add(self.f_completed, id, re_desc.text, True)

    def new(self):
        id = "t" + str(int(datetime.now().timestamp()))
        desc = self.e_desc.get()

        db("data", "todo/desc-" + id, "POST", desc)
        self.add(self.f_active, id, desc, False)

    def add(self, frame, id, desc, checked):
        f_task = tk.Frame(frame)
        f_task.pack(fill="x")

        v_task = tk.BooleanVar()
        v_task.set(checked)

        cb_task = ttk.Checkbutton(f_task, text=desc, variable=v_task, 
            command=lambda: self.completed(f_task, v_task, id, desc)
        )
        cb_task.pack(side=tk.LEFT)

        b_task = ttk.Button(f_task, text="x",
            command=lambda: self.remove(f_task, id)
        )
        b_task.pack(side=tk.RIGHT)

    def completed(self, frame, state, id, desc):
        if  state.get():
            db("data", "todo/ok-" + id, "POST", "1")
            frame.destroy()
            self.add(self.f_completed, id, desc, True)

        else:
            db("data", "todo/ok-" + id, "DELETE")
            frame.destroy()
            self.add(self.f_active, id, desc, False)

    def remove(self, frame, id):
        db("data", "todo/desc-" + id, "DELETE")
        db("data", "todo/ok-" + id, "DELETE")
        frame.destroy()


root = tk.Tk()
app = App(root)
app.mainloop()
