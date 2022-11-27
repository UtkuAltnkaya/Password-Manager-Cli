<h1>Password Manager</h1>
<div>It is a command line interface for managing passwords</div>

<br>
<hr>
<h2>Build Requirements</h2>
<div>
  <ul>
    <li>
      <i>Git</i>
        <ul>
          <small><a>https://git-scm.com/</a></small>
        </ul>
    </li>
    <li>
      <i>Cargo</i>
      <ul>
        <small><a>https://www.rust-lang.org/tools/install</a></small>
      </ul>
    </li>
  </ul>
</div>

<hr>
<h2>Program Requirements</h2>
<div>
  <ul>
    <li>
      <div>
        <i>".env"</i> file
          <ul>
            <small>(You can create your own .env file to exe path)</small>
            <br>
            <small>(Note that you have to add <i>SECRET_KEY = ""</i>)</small>
            <br>
            <small>(You can assign any secret key you want )</small>
          </ul>
      </div>
    </li>
    <li>
      <div>
        <div><i>"table.db"</i> file</div>
        <ul>
          <small>(You can create your own table.db file to db folder in exe path</small>
        </ul>
      </div>
    </li>
  </ul>
  <mark>
    Note that program will be create these file automatically
    <br>
    Do not have to do it manually
  </mark>
</div>
<hr>

<h2>Usage (recommended)</h2>
<br>
<pre>
  git clone <a>https://github.com/UtkuAltnkaya/Password-Manager-Cli.git</a>
  cargo build --release
  cd target
  cd release
</pre>
<div>
  <ul>
    <li>
      <i>copy the path</i>
    </li>
    <li>
      add user environment path
    </li>
  </ul>
</div>
<div>You can access it from anywhere</div>
<hr>

<h2>Usage (not recommended)</h2>
<br>
<pre>
  git clone <a>https://github.com/UtkuAltnkaya/Password-Manager-Cli.git</a>
  cargo build --release
  cd target
  cd release
  ./pm 
</pre>
