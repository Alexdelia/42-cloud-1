use leptos::prelude::*;

#[component]
pub fn StatsBubble(title: String, user: impl IntoView, global: impl IntoView) -> impl IntoView {
	view! {
		<div class="stats_bubble">
			<div class="stats_bubble_title">{title}</div>
			<div class="stats_bubble_user">
				// https://fonts.google.com/icons?selected=Material+Symbols+Rounded:person:FILL@0;wght@400;GRAD@0;opsz@24&icon.query=user&icon.size=24&icon.color=%23e3e3e3&icon.style=Rounded
				<svg
					xmlns="http://www.w3.org/2000/svg"
					height="1.25rem"
					width="1.25rem"
					viewBox="0 -960 960 960"
					fill="#fff"
				>
					<path d="M480-480q-66 0-113-47t-47-113q0-66 47-113t113-47q66 0 113 47t47 113q0 66-47 113t-113 47ZM160-240v-32q0-34 17.5-62.5T224-378q62-31 126-46.5T480-440q66 0 130 15.5T736-378q29 15 46.5 43.5T800-272v32q0 33-23.5 56.5T720-160H240q-33 0-56.5-23.5T160-240Zm80 0h480v-32q0-11-5.5-20T700-306q-54-27-109-40.5T480-360q-56 0-111 13.5T260-306q-9 5-14.5 14t-5.5 20v32Zm240-320q33 0 56.5-23.5T560-640q0-33-23.5-56.5T480-720q-33 0-56.5 23.5T400-640q0 33 23.5 56.5T480-560Zm0-80Zm0 400Z" />
				</svg>
				{user}
			</div>
			<div class="stats_bubble_global">
				// https://fonts.google.com/icons?selected=Material+Symbols+Rounded:language:FILL@0;wght@400;GRAD@0;opsz@24&icon.query=glob&icon.size=24&icon.color=%23e3e3e3&icon.style=Rounded
				<svg
					xmlns="http://www.w3.org/2000/svg"
					height="1.25rem"
					width="1.25rem"
					viewBox="0 -960 960 960"
					fill="#fff"
				>
					<path d="M480-80q-82 0-155-31.5t-127.5-86Q143-252 111.5-325T80-480q0-83 31.5-155.5t86-127Q252-817 325-848.5T480-880q83 0 155.5 31.5t127 86q54.5 54.5 86 127T880-480q0 82-31.5 155t-86 127.5q-54.5 54.5-127 86T480-80Zm0-82q26-36 45-75t31-83H404q12 44 31 83t45 75Zm-104-16q-18-33-31.5-68.5T322-320H204q29 50 72.5 87t99.5 55Zm208 0q56-18 99.5-55t72.5-87H638q-9 38-22.5 73.5T584-178ZM170-400h136q-3-20-4.5-39.5T300-480q0-21 1.5-40.5T306-560H170q-5 20-7.5 39.5T160-480q0 21 2.5 40.5T170-400Zm216 0h188q3-20 4.5-39.5T580-480q0-21-1.5-40.5T574-560H386q-3 20-4.5 39.5T380-480q0 21 1.5 40.5T386-400Zm268 0h136q5-20 7.5-39.5T800-480q0-21-2.5-40.5T790-560H654q3 20 4.5 39.5T660-480q0 21-1.5 40.5T654-400Zm-16-240h118q-29-50-72.5-87T584-782q18 33 31.5 68.5T638-640Zm-234 0h152q-12-44-31-83t-45-75q-26 36-45 75t-31 83Zm-200 0h118q9-38 22.5-73.5T376-782q-56 18-99.5 55T204-640Z" />
				</svg>
				{global}
			</div>
		</div>
	}
}
