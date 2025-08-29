package fr.requet.liftly

import android.os.Bundle
import androidx.core.view.WindowCompat
import androidx.activity.enableEdgeToEdge
import android.webkit.WebView
import android.util.Log
import android.view.KeyEvent

class MainActivity : TauriActivity() {
    private lateinit var wv: WebView

  override fun onCreate(savedInstanceState: Bundle?) {
    Log.i("MainActivity", "onCreate called")
    enableEdgeToEdge()
    super.onCreate(savedInstanceState)
     WindowCompat.setDecorFitsSystemWindows(window, true)
  }

   override fun onWebViewCreate(webView: WebView) {
        super.onWebViewCreate(webView) 
        wv = webView
    }

     private val keyEventMap = mapOf(
        KeyEvent.KEYCODE_BACK to "back"
    )

       override fun onKeyDown(keyCode: Int, event: KeyEvent?): Boolean {
        keyEventMap[keyCode]?.let { keyName ->
            // Use a unique and descriptive function name on the window object
            val script = """
                try {
                    // Returns true if native behavior should proceed, false if handled by webview
                    window.onNativeKeyEvent('$keyName');
                } catch (e) {
                    true; // Default to native behavior if function doesn't exist
                }
            """.trimIndent()

            wv.evaluateJavascript(script) { result ->
                Log.i("MainActivity", "JS result for $keyName: $result")
                if (result != "false") {
                    // When the JS side returns true (or anything but "false"),
                    // we execute the default native behavior.
                    super.onKeyDown(keyCode, event)
                }
                // If the JS side returns "false", we do nothing, effectively
                // consuming the event.
            }
            return true // We've "handled" the event dispatch, JS will decide the outcome
        }
        return super.onKeyDown(keyCode, event)
    }
}
