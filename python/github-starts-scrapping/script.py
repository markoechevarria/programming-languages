from selenium import webdriver
from selenium.webdriver.chrome.options import Options
from selenium.webdriver.common.by import By

opts = Options()
opts.binary_location =  r'/usr/bin/brave'

driver = webdriver.Chrome(options=opts)
driver.get("https://www.github.com")

elements = driver.find_element(By.XPATH, "")
