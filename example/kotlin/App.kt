import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.ui.Modifier
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import co.design.system.Button as DesignSystemButton

internal object MainPage : Page() {
  @Composable
  override fun Content() {
    val counter by remember { mutableStateOf(0) }

    Box(
      modifier = Modifier.fillMaxWidth()
    ) {
      Column {
        DesignSystemButton(
          text = "Some Button",
          onClick = { counter = counter + 1 }
        )
      }
    }
  }
}
