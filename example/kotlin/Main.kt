import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.ui.Modifier
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.foundation.layout.Column
import co.design.system.Button
import co.design.system.layout.Box

internal object MainPage : Page() {
  @Composable
  override fun Content() {
    val counter by remember { mutableStateOf(0) }

    Box(
      modifier = Modifier.fillMaxWidth()
    ) {
      Column {
        Button(
          text = "Some Button",
          onClick = { counter = counter + 1 }
        )
      }
    }
  }
}
