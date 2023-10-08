import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

GetResponse = WS.sendRequest(findTestObject('GET Single User', [('Base_URL') : GlobalVariable.Base_URL, ('id') : GlobalVariable.id]))
def slurper = new groovy.json.JsonSlurper()
def result = slurper.parseText(GetResponse.getResponseBodyContent())

def emailValue = result.data.email

GlobalVariable.email = emailValue
GlobalVariable.password = 'pistol'

response = WS.sendRequest(findTestObject('POST Register', [('Base_URL') : GlobalVariable.Base_URL]))

WS.verifyResponseStatusCode(response, 200, FailureHandling.STOP_ON_FAILURE)

WS.verifyElementPropertyValue(response, 'token', "QpwL5tke4Pnpja7X1")