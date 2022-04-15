package com.arnabanimesh.demorsndk

import android.graphics.Bitmap
import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.widget.ImageView
import android.widget.TextView

class MainActivity : AppCompatActivity() {
    private external fun imgCaptionRs(input: String): String
    private external fun renderJulia(bitmap: Bitmap)
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)
        val textView: TextView = findViewById(R.id.imgCaption)
        textView.text = imgCaptionRs("Julia Set")
        val bitmap = Bitmap.createBitmap(800, 800, Bitmap.Config.ARGB_8888)
        renderJulia(bitmap)
        val image: ImageView = findViewById(R.id.julia)
        image.setImageBitmap(bitmap)
    }
    companion object {
        init {
            System.loadLibrary("rust")
        }
    }
}
