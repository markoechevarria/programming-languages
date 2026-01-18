package com.example.businesscardapp

import android.health.connect.datatypes.ExercisePerformanceGoal
import android.os.Bundle
import android.text.style.BackgroundColorSpan
import androidx.activity.ComponentActivity
import androidx.activity.compose.LocalActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.rounded.Call
import androidx.compose.material.icons.rounded.Email
import androidx.compose.material.icons.rounded.Menu
import androidx.compose.material.icons.rounded.Share
import androidx.compose.material3.Icon
import androidx.compose.material3.LocalContentColor
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.modifier.modifierLocalOf
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.example.businesscardapp.ui.theme.BusinessCardAppTheme

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()
        setContent {
            BusinessCardAppTheme {
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = Color(0xFFd2e9d5)
                ) {
                    WindowFinal()
                }
            }
        }
    }
}

@Composable
fun CentralCard( name: String = "Marko Echevarria", title: String="Android Developer Extraordinaire", modifier: Modifier = Modifier) {
    val image = painterResource(R.drawable.android_logo)
    Column (modifier = modifier,
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.spacedBy(12.dp)
    ) {
        Image(
            painter = image,
            contentDescription = null,
            modifier = modifier.height(150.dp).width(150.dp).background(Color(0xFF073143))
        )
        Text(
            text = name,
            fontSize = 40.sp,
            modifier = modifier,
        )
        Text(
            text = title,
            modifier = modifier,
            fontWeight = FontWeight.Bold,
            color = Color(0xFF147848)
        )
    }
}

@Composable
fun ContactCard(number: String = "+51 999888777", correo: String = "@unmsm.com", email: String = "markofidel@gmail.com", modifier: Modifier = Modifier) {
    Column (
        modifier = modifier.padding(bottom = 20.dp),
        verticalArrangement = Arrangement.spacedBy ( 12.dp )
    ) {
        Row ( modifier = modifier, horizontalArrangement = Arrangement.spacedBy(15.dp)) {
            Icon(
                Icons.Rounded.Call,
                contentDescription = "Call Icon",
                tint = Color(0xFF016d3a),
            )
            Text( text = number, color = Color(0xFF016d3a) )
        }
        Row ( modifier = modifier, horizontalArrangement = Arrangement.spacedBy(15.dp) ) {
            Icon(
                Icons.Rounded.Share,
                contentDescription = "Share Icon",
                tint = Color(0xFF016d3a)
            )
            Text( text = correo, color = Color(0xFF016d3a) )
        }
        Row ( modifier = modifier, horizontalArrangement = Arrangement.spacedBy(15.dp) ) {
            Icon(
                Icons.Rounded.Email,
                contentDescription = "Mail Icon",
                tint = Color(0xFF016d3a)
            )
            Text( text = email, color = Color(0xFF016d3a) )
        }
    }
}

@Composable
fun WindowFinal() {
    Column (
        modifier = Modifier.padding(20.dp),
        verticalArrangement = Arrangement.Bottom,
        horizontalAlignment = Alignment.CenterHorizontally,
    ) {
        CentralCard()
        Spacer(modifier = Modifier.height(200.dp))
        ContactCard()
    }
}