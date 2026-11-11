package it.agrometeo.sardegna

import android.content.ComponentName
import android.content.Intent
import android.content.pm.PackageManager
import android.os.Bundle
import android.view.KeyEvent
import android.webkit.WebView
import androidx.activity.OnBackPressedCallback
import androidx.activity.enableEdgeToEdge

class MainActivity : TauriActivity() {
  private var vista: WebView? = null

  override fun onCreate(savedInstanceState: Bundle?) {
    enableEdgeToEdge()
    super.onCreate(savedInstanceState)
    if (packageManager.hasSystemFeature(PackageManager.FEATURE_LEANBACK)) {
      diventaHomeSeBoxSenzaLauncher()
      indietroDalTelecomando()
    }
  }

  override fun onWebViewCreate(webView: WebView) {
    vista = webView
    // sulla TV la pagina prende subito il fuoco: il primo tasto freccia parte dalla scheda attiva
    if (packageManager.hasSystemFeature(PackageManager.FEATURE_LEANBACK)) webView.requestFocus()
  }

  /**
   * Su un campo di testo il tasto OK del telecomando non raggiunge la pagina: glielo si segnala,
   * così la pagina apre la tastiera a schermo solo quando la si chiede (js/tv.js).
   */
  override fun dispatchKeyEvent(event: KeyEvent): Boolean {
    if (event.action == KeyEvent.ACTION_UP && event.keyCode == KeyEvent.KEYCODE_DPAD_CENTER) {
      vista?.evaluateJavascript("window.__tvOk && window.__tvOk()", null)
    }
    return super.dispatchKeyEvent(event)
  }

  private val aliasHome get() = ComponentName(this, "$packageName.HomeTv")

  private fun eLaHome() =
    packageManager.getComponentEnabledSetting(aliasHome) == PackageManager.COMPONENT_ENABLED_STATE_ENABLED

  /**
   * Su un box TV dedicato, privo di launcher (resta solo la "FallbackHome" di sistema, priorità
   * negativa), l'app accende il proprio alias HOME: il tasto Home del telecomando la riapre.
   * Se in seguito viene installato un launcher vero, l'alias si spegne e l'app torna un'app normale.
   * Su telefoni e TV con un launcher non cambia nulla.
   */
  private fun diventaHomeSeBoxSenzaLauncher() {
    try {
      val home = Intent(Intent.ACTION_MAIN).addCategory(Intent.CATEGORY_HOME)
      val launcherVero = packageManager.queryIntentActivities(home, 0)
        .any { it.activityInfo.packageName != packageName && it.priority >= 0 }
      if (launcherVero != eLaHome()) return
      packageManager.setComponentEnabledSetting(
        aliasHome,
        if (launcherVero) PackageManager.COMPONENT_ENABLED_STATE_DEFAULT else PackageManager.COMPONENT_ENABLED_STATE_ENABLED,
        PackageManager.DONT_KILL_APP
      )
    } catch (_: Exception) {
      // facoltativo: se il sistema lo nega l'app funziona comunque
    }
  }

  /**
   * Sulla TV il tasto Indietro lo gestisce la pagina (js/tv.js): chiude ciò che è aperto o torna
   * alle schede. Se non c'è più nulla da fare l'app si chiude, tranne quando è lei la Home.
   */
  private fun indietroDalTelecomando() {
    onBackPressedDispatcher.addCallback(this, object : OnBackPressedCallback(true) {
      override fun handleOnBackPressed() {
        val v = vista
        if (v == null) {
          if (!eLaHome()) finish()
          return
        }
        v.evaluateJavascript("!!(window.__tvIndietro && window.__tvIndietro())") { gestito ->
          if (gestito != "true" && !eLaHome()) finish()
        }
      }
    })
  }
}
