
#include <gtk/gtk.h>
#include <stdlib.h>

char* open_file_dialog() {
    GtkWidget *dialog;
    GtkFileChooserAction action = GTK_FILE_CHOOSER_ACTION_OPEN;
    gint res;
    char *filename = NULL;

    dialog = gtk_file_chooser_dialog_new("Open File",
                                        NULL,
                                        action,
                                        "_Cancel", GTK_RESPONSE_CANCEL,
                                        "_Open", GTK_RESPONSE_ACCEPT,
                                        NULL);

    res = gtk_dialog_run(GTK_DIALOG(dialog));
    if (res == GTK_RESPONSE_ACCEPT) {
        char *selected_path = gtk_file_chooser_get_filename(GTK_FILE_CHOOSER(dialog));
        filename = strdup(selected_path);
        g_free(selected_path);
    }

    gtk_widget_destroy(dialog);
    return filename;
}

char* open_directory_dialog() {
    GtkWidget *dialog;
    GtkFileChooserAction action = GTK_FILE_CHOOSER_ACTION_SELECT_FOLDER;
    gint res;
    char *filename = NULL;

    dialog = gtk_file_chooser_dialog_new("Select Folder",
                                        NULL,
                                        action,
                                        "_Cancel", GTK_RESPONSE_CANCEL,
                                        "_Select", GTK_RESPONSE_ACCEPT,
                                        NULL);

    res = gtk_dialog_run(GTK_DIALOG(dialog));
    if (res == GTK_RESPONSE_ACCEPT) {
        char *selected_path = gtk_file_chooser_get_filename(GTK_FILE_CHOOSER(dialog));
        filename = strdup(selected_path);
        g_free(selected_path);
    }

    gtk_widget_destroy(dialog);
    return filename;
}
