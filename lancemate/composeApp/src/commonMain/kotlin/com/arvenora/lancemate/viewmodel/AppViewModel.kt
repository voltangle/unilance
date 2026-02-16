package com.arvenora.lancemate.viewmodel

import androidx.compose.material3.SnackbarHostState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.setValue
import androidx.lifecycle.ViewModel
import com.arvenora.lancemate.RootNavTarget

class AppViewModel : ViewModel() {
    val snackbarHostState = SnackbarHostState()
    var rootNavTarget by mutableStateOf(RootNavTarget.LiveData)
}
